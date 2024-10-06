use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};
entrypoint!(my_defi_app);
fn my_defi_app(
    program_id: &[Pubkey],
    account:&[AccountInfo],
    instruction_data: &[u8],

) -> ProgramResult {
    msg!("hello , welcome to our Dapp");
    msg!("token buy karlo pls");
    ok(())
}