/* tslint:disable */
/* eslint-disable */
/**
* accept addresses in hex format & coupon secret key in hex, and generate signature for coupon
* @param {string} contract_address_hex
* @param {string} receiver_address_hex
* @param {string} coupon_secret_hex
* @returns {string}
*/
export function get_coupon_signature(contract_address_hex: string, receiver_address_hex: string, coupon_secret_hex: string): string;
