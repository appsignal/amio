pub use self::pipe::Awakener;

/// Default *nix awakener implementation
mod pipe {
    use {io, Evented, EventSet, PollOpt, Selector, Token, TryRead, TryWrite};
    use unix::{self, PipeReader, PipeWriter};

    /*
     *
     * ===== Awakener =====
     *
     */

    pub struct Awakener {
        reader: PipeReader,
        writer: PipeWriter,
    }

    impl Awakener {
        pub fn new() -> io::Result<Awakener> {
            let (rd, wr) = unix::pipe()?;

            Ok(Awakener {
                reader: rd,
                writer: wr,
            })
        }

        pub fn wakeup(&self) -> io::Result<()> {
            (&self.writer).try_write(b"0x01").map(|_| ())
        }

        pub fn cleanup(&self) {
            let mut buf = [0; 128];

            loop {
                // Consume data until all bytes are purged
                match (&self.reader).try_read(&mut buf) {
                    Ok(Some(i)) if i > 0 => {},
                    _ => return,
                }
            }
        }

        fn reader(&self) -> &PipeReader {
            &self.reader
        }
    }

    impl Evented for Awakener {
        fn register(&self, selector: &mut Selector, token: Token, interest: EventSet, opts: PollOpt) -> io::Result<()> {
            self.reader().register(selector, token, interest, opts)
        }

        fn reregister(&self, selector: &mut Selector, token: Token, interest: EventSet, opts: PollOpt) -> io::Result<()> {
            self.reader().reregister(selector, token, interest, opts)
        }

        fn deregister(&self, selector: &mut Selector) -> io::Result<()> {
            self.reader().deregister(selector)
        }
    }
}
