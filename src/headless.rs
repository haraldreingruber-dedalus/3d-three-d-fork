use crate::Context;
use glutin::event_loop::EventLoop;
use glutin::ContextBuilder;
use glutin::*;
use glutin::dpi::PhysicalSize;

pub struct HeadlessContext {
    _headless_context: glutin::Context<PossiblyCurrent>,
    gl: crate::Context,
}

impl HeadlessContext {
    ///
    /// Constructs a new headless context
    ///
    pub fn new() -> Result<HeadlessContext, ()> {
        // inspired by https://github.com/rust-windowing/glutin/blob/bab33a84dfb094ff65c059400bed7993434638e2/glutin_examples/examples/headless.rs#L80-L87
        let cb = ContextBuilder::new();
        let (headless_context, _el) = build_context(cb).unwrap();
        let headless_context = unsafe { headless_context.make_current().unwrap() };

        let gl = Context::load_with(|ptr| headless_context.get_proc_address(ptr) as *const std::os::raw::c_void);

        Ok(HeadlessContext {
            _headless_context: headless_context,
            gl,
        })
    }

    ///
    /// Returns the graphics context for this window.
    ///
    pub fn gl(&self) -> Result<crate::Context, ()> {
        Ok(self.gl.clone())
    }
}


#[cfg(target_os = "linux")]
fn build_context_surfaceless<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
    el: &EventLoop<()>,
) -> Result<Context<NotCurrent>, CreationError> {
    use glutin::platform::unix::HeadlessContextExt;
    cb.build_surfaceless(&el)
}

fn build_context_headless<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
    el: &EventLoop<()>,
) -> Result<glutin::Context<NotCurrent>, CreationError> {
    let size_one = PhysicalSize::new(1, 1);
    cb.build_headless(&el, size_one)
}

#[cfg(target_os = "linux")]
fn build_context_osmesa<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
) -> Result<Context<NotCurrent>, CreationError> {
    use glutin::platform::unix::HeadlessContextExt;
    let size_one = PhysicalSize::new(1, 1);
    cb.build_osmesa(size_one)
}

#[cfg(target_os = "linux")]
fn build_context<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
) -> Result<(Context<NotCurrent>, EventLoop<()>), [CreationError; 3]> {
    // On unix operating systems, you should always try for surfaceless first,
    // and if that does not work, headless (pbuffers), and if that too fails,
    // finally osmesa.
    //
    // If willing, you could attempt to use hidden windows instead of os mesa,
    // but note that you must handle events for the window that come on the
    // events loop.
    let el = EventLoop::new();

    println!("Trying surfaceless");
    let err1 = match build_context_surfaceless(cb.clone(), &el) {
        Ok(ctx) => return Ok((ctx, el)),
        Err(err) => err,
    };

    println!("Trying headless");
    let err2 = match build_context_headless(cb.clone(), &el) {
        Ok(ctx) => return Ok((ctx, el)),
        Err(err) => err,
    };

    println!("Trying osmesa");
    let err3 = match build_context_osmesa(cb) {
        Ok(ctx) => return Ok((ctx, el)),
        Err(err) => err,
    };

    Err([err1, err2, err3])
}

#[cfg(not(target_os = "linux"))]
fn build_context<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
) -> Result<(glutin::Context<NotCurrent>, EventLoop<()>), CreationError> {
    let el = EventLoop::new();
    build_context_headless(cb.clone(), &el).map(|ctx| (ctx, el))
}

