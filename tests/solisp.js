const anchor = require('@project-serum/anchor');

describe('solisp', () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env()
  anchor.setProvider(provider);

  it('Is initialized!', async () => {
    // Add your test here.
    const program = anchor.workspace.Solisp;
    const tx = await program.rpc.run('(* (+ 1 2) 3)');
    console.log("Your transaction signature", tx);
  });
});
