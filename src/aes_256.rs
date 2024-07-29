use super::common::*;

const KEY_LENGTH: usize = 8;
const ROUNDS: usize = 14;
const KEY_COUNT: usize = (ROUNDS + 1) * 4 / KEY_LENGTH + 1;

type Key = [u8; 4 * KEY_LENGTH];
type KeySchedule = [Block; ROUNDS + 1];

pub fn encrypt_block(input: Block, key: Key) -> Block {
	let w = key_expansion(key);

	let mut state: Block = input;

	state = add_round_key(state, &w[0]);

	for i in 1..ROUNDS {
		state = sub_bytes(state);
		state = shift_rows(state);
		state = mix_columns(state);
		state = add_round_key(state, &w[i]);
	}

	state = sub_bytes(state);
	state = shift_rows(state);
	state = add_round_key(state, &w[ROUNDS]);

	state
}

pub fn decrypt_block(input: Block, key: Key) -> Block {
	let w = key_expansion(key);

	let mut state: Block = input;

	state = add_round_key(state, &w[ROUNDS]);

	let mut r = ROUNDS - 1;
	while r > 0 {
		state = inv_shift_rows(state);
		state = inv_sub_bytes(state);
		state = add_round_key(state, &w[r]);
		state = inv_mix_columns(state);

		r = r - 1;
	}

	state = inv_shift_rows(state);
	state = inv_sub_bytes(state);
	state = add_round_key(state, &w[0]);

	state
}

pub fn key_expansion(key: Key) -> KeySchedule {
	let mut w = [[0; 4 * KEY_LENGTH]; KEY_COUNT];

	w[0] = key;

	let mut temp: [u8; 4] = as_2d(&w[0])[KEY_LENGTH - 1];
	for r in 1..KEY_COUNT {
		let word = sub_word(rot_word(temp));
		temp[0] = word[0] ^ R_CON[r];
		temp[1] = word[1] ^ 0x00;
		temp[2] = word[2] ^ 0x00;
		temp[3] = word[3] ^ 0x00;
		for k in 0..4 {
			for x in 0..4 {
				w[r][x + k * 4] = w[r - 1][x + k * 4] ^ temp[x];
			}
			temp = as_2d(&w[r])[k];
		}
		temp = sub_word(temp);
		for k in 4..8 {
			for x in 0..4 {
				w[r][x + k * 4] = w[r - 1][x + k * 4] ^ temp[x];
			}
			temp = as_2d(&w[r])[k];
		}
	}

	*(unsafe { &*(w.as_ptr() as *const KeySchedule) })
}

pub fn as_2d(key: &Key) -> &[[u8; 4]; KEY_LENGTH] {
	unsafe { &*(key.as_ptr() as *const [[u8; 4]; KEY_LENGTH]) }
}
