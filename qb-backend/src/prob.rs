// SPDX-License-Identifier: AGPL-3.0-only

// ████████████████████████████████████████████████
// █─▄▄▄─█▄─██─▄█▄─▄█▄─▀─▄█▄─▄─▀█▄─█─▄█─▄─▄─█▄─▄▄─█
// █─██▀─██─██─███─███▀─▀███─▄─▀██▄─▄████─████─▄█▀█
// ▀───▄▄▀▀▄▄▄▄▀▀▄▄▄▀▄▄█▄▄▀▄▄▄▄▀▀▀▄▄▄▀▀▀▄▄▄▀▀▄▄▄▄▄▀
// https://github.com/QuixByte/qb/blob/main/LICENSE

// (c) Copyright 2023 The QuixByte Authors

use paste::paste;

macro_rules! domain {
    ($domain:ident) => {
        paste! {
            pub const $domain: &'static str = concat!("https://quixbyte.com/probs/", stringify!([<$domain:lower>]));
        }
    }
}

domain!(INVALID_CREDENTIALS);
domain!(INVALID_NAME);
domain!(INVALID_DISPLAY_NAME);
domain!(INVALID_PAYLOAD);
domain!(CONFLICT_USER);
domain!(NOT_FOUND);
