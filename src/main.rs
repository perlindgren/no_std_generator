#![feature(generator_trait)]
#![feature(generators)]
#![feature(never_type)]
#![feature(type_alias_impl_trait)]

use core::{mem::MaybeUninit, ops::Generator, pin::Pin};

#[rustfmt::skip]
type G = impl Generator<Yield = &'static u32, Return = !>;

fn task(x: &'static mut u32) -> G {
    static mut D: u32 = 0;
    move || loop {
        println!("Hello {} {}", &x, unsafe { D });
        *x += 1;
        unsafe {
            D += 2;
        }
        yield (unsafe { &D });

        println!("World {} {}", &x, unsafe { D });
        unsafe {
            D += 2;
        }
        yield (unsafe { &D });
    }
}

static mut X: MaybeUninit<G> = MaybeUninit::uninit();

fn main() {
    unsafe {
        static mut x: u32 = 0;
        X.as_mut_ptr().write(task(&mut x));
        let g: &mut dyn Generator<Yield = &'static u32, Return = !> =
            &mut *X.as_mut_ptr();
        println!("-- {:?}", Pin::new_unchecked(&mut *g).resume());
        println!("-- {:?}", Pin::new_unchecked(&mut *g).resume());
        println!("-- {:?}", Pin::new_unchecked(&mut *g).resume());
    }
}
