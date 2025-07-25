#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        type SomeReceiver;

        #[namespace = "this_will_be_ignored_and_therefore_should_be_an_error"]
        fn some_method(self: &SomeReceiver) -> bool;
    }
}

fn main() {}
