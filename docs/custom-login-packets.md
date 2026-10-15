# Custom login packets

These packets are not part of the client. A custom client or bot sends them to the login port, the same connection that takes `CA_LOGIN` (`0x0064`). The official client never sends them.

## `CA_CREATE_ACCOUNT` (`0x0A10`, client to login)

| Offset | Size | Field                                        |
|--------|------|----------------------------------------------|
| 0      | 2    | packet id, `10 0A`                           |
| 2      | 24   | user id, NUL padded                          |
| 26     | 24   | password, NUL padded                         |
| 50     | 1    | sex, `M` or `F` (lower case is accepted)     |

Total 51 bytes, no length field. The packet creates an account without logging in and uses the same rules as the `name_M` registration: `login.new_account` must be on, the name and password minimum lengths apply, and `allowed_regs` / `time_allowed` limit the flood. A name may only use letters, digits and `_`, and it may not end in `_M` or `_F`, because the login path would read that suffix as a sex.

## `AC_CREATE_ACCOUNT_RESULT` (`0x0A11`, login to client)

| Offset | Size | Field                                                 |
|--------|------|-------------------------------------------------------|
| 0      | 2    | packet id, `11 0A`                                    |
| 2      | 1    | result code, `0` when created                         |
| 3      | 4    | account id (little endian), `0` when refused          |

Result codes:

| Code | Meaning                                                              |
|------|----------------------------------------------------------------------|
| 0    | created                                                              |
| 1    | account creation is off (`login.new_account` is false)               |
| 2    | invalid name: shorter than `acc_name_min_length`, a character outside letters, digits and `_`, or a `_M` / `_F` suffix |
| 3    | invalid password: empty or shorter than `password_min_length`         |
| 4    | invalid sex                                                          |
| 5    | name already taken                                                   |
| 6    | registration limit reached (`allowed_regs` in `time_allowed` seconds) |
| 7    | storage failure                                                      |
| 8    | the address is IP banned (`ipban_enable`)                            |

## `CA_SERVER_INFO` (`0x0A27`, client to login) and `AC_SERVER_INFO` (`0x0A28`, login to client)

The request has no body, only the 2-byte header. The answer describes the connection that asked:

| Offset | Size | Field                                                              |
|--------|------|--------------------------------------------------------------------|
| 0      | 2    | packet id, `28 0A`                                                 |
| 2      | 1    | accounts required, `1` unless the caller's address is in `skip_account_ips` |
| 3      | 1    | account creation on, `1` when `login.new_account` is on           |
| 4      | 4    | packet version of the server (little endian)                       |

Total 8 bytes. A bot can send it before `CA_LOGIN` to decide whether to show a password field or to create an account first. A skipped address accepts any password, so it does not need to ask.

## Skipping the account system: `login.skip_account_ips`

Clients from an address in `skip_account_ips` log in without an account. The list takes `1.2.3.4` or a pattern with `*` octets (`10.0.0.*`), the same syntax as IP bans.

For a skipped address, `CA_LOGIN` does this:

- the password is not checked, and nothing else in the account record is checked either: bans, expiry, the `state` block, and the `group_id_to_connect` gates are all skipped;
- an unknown login id creates an account on first use. The name is the login id without a `_M` / `_F` suffix, and the sex is taken from the suffix (`M` when there is none). The account gets a random password nobody knows, so it can only be entered from a skipped address;
- server accounts (sex `S`) are still refused;
- IP bans still apply, because they belong to the address, not to the account.

Leave the list empty (the default) to keep the account system on every address.
