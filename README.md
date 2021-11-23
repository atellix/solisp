Lisp on Solana

Based on a adapted version of: [Brundon Smith's rust_lisp](https://github.com/brundonsmith/rust_lisp/)

This is an experiment to bring dynamic languages to the Solana blockchain for smart contracts.

*Use at your own risk!*

Right now all Lisp interpretation happens on-chain, but this might change in the future.

Also, currently the command capabilities are extremely limited. More commands may be added in the future.

This is mostly a demonstration that the concept works, but there is a basic road map which might make this a useful language.

- [ ] Create a Lisp bytecode that can be sent as a binary blob.
- [ ] Add the ability to store Lisp bytecode in accounts that can be used later.
- [ ] Pre-process Lisp programs to automatically generate the Solana accounts list.
- [ ] Add lots more commands.
