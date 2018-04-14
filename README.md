notmuch-rs
==========

This is not much more than a wrapper for the [notmuch](https://notmuchmail.org/) C api.

## Building
**notmuch-rs** expects libnotmuch development files to be installed on your system.


## Using

Add this to your `Cargo.toml`:

```toml
[dependencies]
notmuch = "*"
```

and this to your crate root:

```rust
extern crate notmuch;
```

## Example

```rust
extern crate notmuch;

fn main() {

    let mut mail_path = std::env::home_dir().unwrap();
    mail_path.push(".mail");

    let db = notmuch::Database::open(&mail_path.to_str().unwrap().to_string(), notmuch::DatabaseMode::ReadOnly).unwrap();
    let query = db.create_query(&"".to_string()).unwrap();
    let mut threads = query.search_threads().unwrap();

    loop {
        match threads.next() {
            Some(thread) => {
                println!("thread {:?} {:?}", thread.subject(), thread.authors());
            },
            None => { break }
        }
    }
}

```

## Concurrency

Notmuch makes no claims regarding thread safety. It does not seem to use any
thread locals, but I did not spot any locks. So, as far as I am concerned, it is
not thread safe.  
So why do all structs implement ```Send``` and ```Sync```? Well, it _is_ safe to
access pointers from different threads (as long as you know what you are doing :) ).
But, more importantly, all structs are strictly linked together with their
lifetime. The root of the tree is ```notmuch::Database```, which has a lifetime
that must outlive any related objects, for instance ```notmuch::Query```. The
```notmuch::Threads``` iterator that you can get from a ```notmuch::Query``` is
always outlived by the parent query.  
This means that you can only use these structs accross thread bounds if you
figure out how to satisfy the lifetime requirements. Up until now, I haven't
been able to do that (though my knowledge of Rust is still rather basic).  
So, concurrency seems currently limited to scoped threads.

## Acknowledgements

notmuch-rs started out from the following projects:
 - https://github.com/Stebalien/notmuch-sys/blob/master/src/lib.rs
 - https://github.com/cmhamill/rust-notmuch

Any contributions are welcome!