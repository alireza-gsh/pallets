// Copyright (c) 2019 Alain Brenzikofer
// This file is part of Encointer
//
// Encointer is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Encointer is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Encointer.  If not, see <http://www.gnu.org/licenses/>.

use super::*;
use crate::mock::{new_test_ext, Origin, TestRuntime};
use frame_support::assert_ok;
use sp_core::H256;
use xcm_executor::traits::Convert;

use test_utils::*;

type SybilGate = crate::Pallet<TestRuntime>;

#[test]
fn faucet_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(SybilGate::request_personhood_uniqueness_rating(
			Origin::signed(AccountKeyring::Alice.into()),
			2,
			1,
			Default::default(),
			SybilResponse::default()
		));
	})
}

#[test]
fn faucet_returns_err_if_proof_too_weak() {
	let sibling = sibling_junction(1863);
	let account = LocationConverter::convert_ref(&sibling.clone().into()).unwrap();
	let alice: AccountId = AccountKeyring::Alice.into();
	let request_hash = H256::default();

	new_test_ext().execute_with(|| {
		PendingRequests::<TestRuntime>::insert(request_hash, &alice);

		assert_eq!(
			SybilGate::faucet(
				Origin::signed(account),
				request_hash,
				PersonhoodUniquenessRating::default()
			)
			.unwrap_err(),
			Error::<TestRuntime>::PersonhoodUniquenessRatingTooWeak.into()
		);
	})
}

#[test]
fn faucet_returns_err_for_unexpected_request() {
	let sibling = sibling_junction(1863);
	let account = LocationConverter::convert_ref(&sibling.clone().into()).unwrap();

	new_test_ext().execute_with(|| {
		assert!(SybilGate::faucet(
			Origin::signed(account),
			Default::default(),
			PersonhoodUniquenessRating::default()
		)
		.is_err());
	})
}

#[test]
fn test_enum_encode() {
	assert_eq!(SybilResponse::Faucet as u8, 1);
}
