// Generated with Sol2Ink v1.0.0
// https://github.com/Supercolony-net/sol2ink

pub use crate::{
    impls,
    impls::Internal as _,
    traits::*,
};
use ink_prelude::{
    string::String,
    vec::Vec,
};
use openbrush::{
    storage::Mapping,
    traits::{
        AccountId,
        AccountIdExt,
        Storage,
        ZERO_ADDRESS,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    ///Token name
    pub name: String,
    ///Token symbol
    pub symbol: String,
    ///Mapping from token ID to owner address
    pub owners: Mapping<u128, AccountId>,
    ///Mapping owner address to token count
    pub balances: Mapping<AccountId, u128>,
    ///Mapping from token ID to approved address
    pub token_approvals: Mapping<u128, AccountId>,
    ///Mapping from owner to operator approvals
    pub operator_approvals: Mapping<(AccountId, AccountId), bool>,
    pub _reserved: Option<()>,
}
impl<T: Storage<Data>> ERC721 for T {
    /// @dev See {IERC165-supportsInterface}.
    fn supports_interface(&self, interface_id: [u8; 4]) -> Result<bool, Error> {
        return Ok(interface_id == ierc_721.interface_id
            || interface_id == ierc_721_metadata.interface_id
            || super.supports_interface(interface_id)?)
    }

    /// @dev See {IERC721-balanceOf}.
    fn balance_of(&self, owner: AccountId) -> Result<u128, Error> {
        if owner.is_zero() {
            return Err(Error::Custom(String::from(
                "ERC721: address zero is not a valid owner",
            )))
        }
        return Ok(self.data().balances.get(&owner).unwrap_or_default())
    }

    /// @dev See {IERC721-ownerOf}.
    fn owner_of(&self, token_id: u128) -> Result<AccountId, Error> {
        let mut owner: AccountId = self.data().owners.get(&token_id).unwrap_or_default();
        if owner.is_zero() {
            return Err(Error::Custom(String::from("ERC721: invalid token ID")))
        }
        return Ok(owner)
    }

    /// @dev See {IERC721Metadata-name}.
    fn name(&self) -> Result<String, Error> {
        return Ok(self.data().name)
    }

    /// @dev See {IERC721Metadata-symbol}.
    fn symbol(&self) -> Result<String, Error> {
        return Ok(self.data().symbol)
    }

    /// @dev See {IERC721Metadata-tokenURI}.
    fn token_uri(&self, token_id: u128) -> Result<String, Error> {
        self._require_minted(token_id)?;
        let mut base_uri: String = self._base_uri()?;
        return Ok(if Vec::<u8>::from(base_uri).length > 0 {
            (abi.encode_packed(base_uri, token_id.to_string()?)? as String)
        } else {
            ""
        })
    }

    /// @dev See {IERC721-approve}.
    fn approve(&mut self, to: AccountId, token_id: u128) -> Result<(), Error> {
        let mut owner: AccountId = erc_721.owner_of(token_id)?;
        if to == owner {
            return Err(Error::Custom(String::from(
                "ERC721: approval to current owner",
            )))
        }
        if Self::env().caller() != owner || self.is_approved_for_all(owner, msg.sender)? {
            return Err(Error::Custom(String::from(
                "ERC721: approve caller is not token owner nor approved for all",
            )))
        }
        self._approve(to, token_id)?;
        Ok(())
    }

    /// @dev See {IERC721-getApproved}.
    fn get_approved(&self, token_id: u128) -> Result<AccountId, Error> {
        self._require_minted(token_id)?;
        return Ok(self
            .data()
            .token_approvals
            .get(&token_id)
            .unwrap_or_default())
    }

    /// @dev See {IERC721-setApprovalForAll}.
    fn set_approval_for_all(&mut self, operator: AccountId, approved: bool) -> Result<(), Error> {
        self._set_approval_for_all(Self::env().caller(), operator, approved)?;
        Ok(())
    }

    /// @dev See {IERC721-isApprovedForAll}.
    fn is_approved_for_all(&self, owner: AccountId, operator: AccountId) -> Result<bool, Error> {
        return Ok(self
            .data()
            .operator_approvals
            .get(&(owner, operator))
            .unwrap_or_default())
    }

    /// @dev See {IERC721-transferFrom}.
    fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
    ) -> Result<(), Error> {
        // solhint-disable-next-line max-line-length
        if !self._is_approved_or_owner(Self::env().caller(), token_id)? {
            return Err(Error::Custom(String::from(
                "ERC721: caller is not token owner nor approved",
            )))
        }
        self._transfer(from, to, token_id)?;
        Ok(())
    }

    /// @dev See {IERC721-safeTransferFrom}.
    fn safe_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
    ) -> Result<(), Error> {
        self.safe_transfer_from(from, to, token_id, "")?;
        Ok(())
    }

    /// @dev See {IERC721-safeTransferFrom}.
    fn safe_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        if !self._is_approved_or_owner(Self::env().caller(), token_id)? {
            return Err(Error::Custom(String::from(
                "ERC721: caller is not token owner nor approved",
            )))
        }
        self._safe_transfer(from, to, token_id, data)?;
        Ok(())
    }

    fn name(&self) -> String {
        self.data().name
    }
    fn symbol(&self) -> String {
        self.data().symbol
    }
    fn owners(&self) -> Mapping<u128, AccountId> {
        self.data().owners
    }
    fn balances(&self) -> Mapping<AccountId, u128> {
        self.data().balances
    }
    fn token_approvals(&self) -> Mapping<u128, AccountId> {
        self.data().token_approvals
    }
    fn operator_approvals(&self) -> Mapping<(AccountId, AccountId), bool> {
        self.data().operator_approvals
    }
}

pub trait Internal {
    /// @dev Base URI for computing {tokenURI}. If set, the resulting URI for each
    /// token will be the concatenation of the `baseURI` and the `tokenId`. Empty
    /// by default, can be overridden in child contracts.
    fn _base_uri(&self) -> Result<String, Error>;

    /// @dev Safely transfers `tokenId` token from `from` to `to`, checking first that contract recipients
    /// are aware of the ERC721 protocol to prevent tokens from being forever locked.
    /// `data` is additional data, it has no specified format and it is sent in call to `to`.
    /// This internal function is equivalent to {safeTransferFrom}, and can be used to e.g.
    /// implement alternative mechanisms to perform token transfer, such as signature-based.
    /// Requirements:
    /// - `from` cannot be the zero address.
    /// - `to` cannot be the zero address.
    /// - `tokenId` token must exist and be owned by `from`.
    /// - If `to` refers to a smart contract, it must implement {IERC721Receiver-onERC721Received}, which is called upon a safe transfer.
    /// Emits a {Transfer} event.
    fn _safe_transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
        data: Vec<u8>,
    ) -> Result<(), Error>;

    /// @dev Returns whether `tokenId` exists.
    /// Tokens can be managed by their owner or approved accounts via {approve} or {setApprovalForAll}.
    /// Tokens start existing when they are minted (`_mint`),
    /// and stop existing when they are burned (`_burn`).
    fn _exists(&self, token_id: u128) -> Result<bool, Error>;

    /// @dev Returns whether `spender` is allowed to manage `tokenId`.
    /// Requirements:
    /// - `tokenId` must exist.
    fn _is_approved_or_owner(&self, spender: AccountId, token_id: u128) -> Result<bool, Error>;

    /// @dev Safely mints `tokenId` and transfers it to `to`.
    /// Requirements:
    /// - `tokenId` must not exist.
    /// - If `to` refers to a smart contract, it must implement {IERC721Receiver-onERC721Received}, which is called upon a safe transfer.
    /// Emits a {Transfer} event.
    fn _safe_mint(&mut self, to: AccountId, token_id: u128) -> Result<(), Error>;

    /// @dev Same as {xref-ERC721-_safeMint-address-uint256-}[`_safeMint`], with an additional `data` parameter which is
    /// forwarded in {IERC721Receiver-onERC721Received} to contract recipients.
    fn _safe_mint(&mut self, to: AccountId, token_id: u128, data: Vec<u8>) -> Result<(), Error>;

    /// @dev Mints `tokenId` and transfers it to `to`.
    /// WARNING: Usage of this method is discouraged, use {_safeMint} whenever possible
    /// Requirements:
    /// - `tokenId` must not exist.
    /// - `to` cannot be the zero address.
    /// Emits a {Transfer} event.
    fn _mint(&mut self, to: AccountId, token_id: u128) -> Result<(), Error>;

    /// @dev Destroys `tokenId`.
    /// The approval is cleared when the token is burned.
    /// Requirements:
    /// - `tokenId` must exist.
    /// Emits a {Transfer} event.
    fn _burn(&mut self, token_id: u128) -> Result<(), Error>;

    /// @dev Transfers `tokenId` from `from` to `to`.
    /// As opposed to {transferFrom}, this imposes no restrictions on msg.sender.
    /// Requirements:
    /// - `to` cannot be the zero address.
    /// - `tokenId` token must be owned by `from`.
    /// Emits a {Transfer} event.
    fn _transfer(&mut self, from: AccountId, to: AccountId, token_id: u128) -> Result<(), Error>;

    /// @dev Approve `to` to operate on `tokenId`
    /// Emits an {Approval} event.
    fn _approve(&mut self, to: AccountId, token_id: u128) -> Result<(), Error>;

    /// @dev Approve `operator` to operate on all of `owner` tokens
    /// Emits an {ApprovalForAll} event.
    fn _set_approval_for_all(
        &mut self,
        owner: AccountId,
        operator: AccountId,
        approved: bool,
    ) -> Result<(), Error>;

    /// @dev Reverts if the `tokenId` has not been minted yet.
    fn _require_minted(&self, token_id: u128) -> Result<(), Error>;

    /// @dev Internal function to invoke {IERC721Receiver-onERC721Received} on a target address.
    /// The call is not executed if the target address is not a contract.
    /// @param from address representing the previous owner of the given token ID
    /// @param to target address that will receive the tokens
    /// @param tokenId uint256 ID of the token to be transferred
    /// @param data bytes optional data to send along with the call
    /// @return bool whether the call correctly returned the expected magic value
    fn _check_on_erc_721_received(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
        data: Vec<u8>,
    ) -> Result<bool, Error>;

    /// @dev Hook that is called before any token transfer. This includes minting
    /// and burning.
    /// Calling conditions:
    /// - When `from` and `to` are both non-zero, ``from``'s `tokenId` will be
    /// transferred to `to`.
    /// - When `from` is zero, `tokenId` will be minted for `to`.
    /// - When `to` is zero, ``from``'s `tokenId` will be burned.
    /// - `from` and `to` are never both zero.
    /// To learn more about hooks, head to xref:ROOT:extending-contracts.adoc#using-hooks[Using Hooks].
    fn _before_token_transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
    ) -> Result<(), Error>;

    /// @dev Hook that is called after any transfer of tokens. This includes
    /// minting and burning.
    /// Calling conditions:
    /// - when `from` and `to` are both non-zero.
    /// - `from` and `to` are never both zero.
    /// To learn more about hooks, head to xref:ROOT:extending-contracts.adoc#using-hooks[Using Hooks].
    fn _after_token_transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
    ) -> Result<(), Error>;

    fn _emit_transfer(&self, from: AccountId, to: AccountId, token_id: u128);

    fn _emit_approval(&self, owner: AccountId, approved: AccountId, token_id: u128);

    fn _emit_approval_for_all(&self, owner: AccountId, operator: AccountId, approved: bool);

}

impl<T: Storage<Data>> Internal for T {
    /// @dev Base URI for computing {tokenURI}. If set, the resulting URI for each
    /// token will be the concatenation of the `baseURI` and the `tokenId`. Empty
    /// by default, can be overridden in child contracts.
    default fn _base_uri(&self) -> Result<String, Error> {
        return Ok("")
    }

    /// @dev Safely transfers `tokenId` token from `from` to `to`, checking first that contract recipients
    /// are aware of the ERC721 protocol to prevent tokens from being forever locked.
    /// `data` is additional data, it has no specified format and it is sent in call to `to`.
    /// This internal function is equivalent to {safeTransferFrom}, and can be used to e.g.
    /// implement alternative mechanisms to perform token transfer, such as signature-based.
    /// Requirements:
    /// - `from` cannot be the zero address.
    /// - `to` cannot be the zero address.
    /// - `tokenId` token must exist and be owned by `from`.
    /// - If `to` refers to a smart contract, it must implement {IERC721Receiver-onERC721Received}, which is called upon a safe transfer.
    /// Emits a {Transfer} event.
    default fn _safe_transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        self._transfer(from, to, token_id)?;
        if !self._check_on_erc_721_received(from, to, token_id, data)? {
            return Err(Error::Custom(String::from(
                "ERC721: transfer to non ERC721Receiver implementer",
            )))
        }
        Ok(())
    }

    /// @dev Returns whether `tokenId` exists.
    /// Tokens can be managed by their owner or approved accounts via {approve} or {setApprovalForAll}.
    /// Tokens start existing when they are minted (`_mint`),
    /// and stop existing when they are burned (`_burn`).
    default fn _exists(&self, token_id: u128) -> Result<bool, Error> {
        return Ok(!self
            .data()
            .owners
            .get(&token_id)
            .unwrap_or_default()
            .is_zero())
    }

    /// @dev Returns whether `spender` is allowed to manage `tokenId`.
    /// Requirements:
    /// - `tokenId` must exist.
    default fn _is_approved_or_owner(
        &self,
        spender: AccountId,
        token_id: u128,
    ) -> Result<bool, Error> {
        let mut owner: AccountId = erc_721.owner_of(token_id)?;
        return Ok((spender == owner
            || self.is_approved_for_all(owner, spender)?
            || self.get_approved(token_id)? == spender))
    }

    /// @dev Safely mints `tokenId` and transfers it to `to`.
    /// Requirements:
    /// - `tokenId` must not exist.
    /// - If `to` refers to a smart contract, it must implement {IERC721Receiver-onERC721Received}, which is called upon a safe transfer.
    /// Emits a {Transfer} event.
    default fn _safe_mint(&mut self, to: AccountId, token_id: u128) -> Result<(), Error> {
        self._safe_mint(to, token_id, "")?;
        Ok(())
    }

    /// @dev Same as {xref-ERC721-_safeMint-address-uint256-}[`_safeMint`], with an additional `data` parameter which is
    /// forwarded in {IERC721Receiver-onERC721Received} to contract recipients.
    default fn _safe_mint(
        &mut self,
        to: AccountId,
        token_id: u128,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        self._mint(to, token_id)?;
        if !self._check_on_erc_721_received(ZERO_ADDRESS.into(), to, token_id, data)? {
            return Err(Error::Custom(String::from(
                "ERC721: transfer to non ERC721Receiver implementer",
            )))
        }
        Ok(())
    }

    /// @dev Mints `tokenId` and transfers it to `to`.
    /// WARNING: Usage of this method is discouraged, use {_safeMint} whenever possible
    /// Requirements:
    /// - `tokenId` must not exist.
    /// - `to` cannot be the zero address.
    /// Emits a {Transfer} event.
    default fn _mint(&mut self, to: AccountId, token_id: u128) -> Result<(), Error> {
        if to.is_zero() {
            return Err(Error::Custom(String::from(
                "ERC721: mint to the zero address",
            )))
        }
        if self._exists(token_id)? {
            return Err(Error::Custom(String::from("ERC721: token already minted")))
        }
        self._before_token_transfer(ZERO_ADDRESS.into(), to, token_id)?;
        self.data().balances.insert(
            &to,
            &(self.data().balances.get(&to).unwrap_or_default() + 1),
        );
        self.data().owners.insert(&token_id, &(to));
        self._emit_transfer(ZERO_ADDRESS.into(), to, token_id);
        self._after_token_transfer(ZERO_ADDRESS.into(), to, token_id)?;
        Ok(())
    }

    /// @dev Destroys `tokenId`.
    /// The approval is cleared when the token is burned.
    /// Requirements:
    /// - `tokenId` must exist.
    /// Emits a {Transfer} event.
    default fn _burn(&mut self, token_id: u128) -> Result<(), Error> {
        let mut owner: AccountId = erc_721.owner_of(token_id)?;
        self._before_token_transfer(owner, ZERO_ADDRESS.into(), token_id)?;
        // Clear approvals
        // Sol2Ink Not Implemented yet: delete _tokenApprovals[tokenId];
        self.data().balances.insert(
            &owner,
            &(self.data().balances.get(&owner).unwrap_or_default() - 1),
        );
        // Sol2Ink Not Implemented yet: delete _owners[tokenId];
        self._emit_transfer(owner, ZERO_ADDRESS.into(), token_id);
        self._after_token_transfer(owner, ZERO_ADDRESS.into(), token_id)?;
        Ok(())
    }

    /// @dev Transfers `tokenId` from `from` to `to`.
    /// As opposed to {transferFrom}, this imposes no restrictions on msg.sender.
    /// Requirements:
    /// - `to` cannot be the zero address.
    /// - `tokenId` token must be owned by `from`.
    /// Emits a {Transfer} event.
    default fn _transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
    ) -> Result<(), Error> {
        if erc_721.owner_of(token_id)? != from {
            return Err(Error::Custom(String::from(
                "ERC721: transfer from incorrect owner",
            )))
        }
        if to.is_zero() {
            return Err(Error::Custom(String::from(
                "ERC721: transfer to the zero address",
            )))
        }
        self._before_token_transfer(from, to, token_id)?;
        // Clear approvals from the previous owner
        // Sol2Ink Not Implemented yet: delete _tokenApprovals[tokenId];
        self.data().balances.insert(
            &from,
            &(self.data().balances.get(&from).unwrap_or_default() - 1),
        );
        self.data().balances.insert(
            &to,
            &(self.data().balances.get(&to).unwrap_or_default() + 1),
        );
        self.data().owners.insert(&token_id, &(to));
        self._emit_transfer(from, to, token_id);
        self._after_token_transfer(from, to, token_id)?;
        Ok(())
    }

    /// @dev Approve `to` to operate on `tokenId`
    /// Emits an {Approval} event.
    default fn _approve(&mut self, to: AccountId, token_id: u128) -> Result<(), Error> {
        self.data().token_approvals.insert(&token_id, &(to));
        self._emit_approval(erc_721.owner_of(token_id)?, to, token_id);
        Ok(())
    }

    /// @dev Approve `operator` to operate on all of `owner` tokens
    /// Emits an {ApprovalForAll} event.
    default fn _set_approval_for_all(
        &mut self,
        owner: AccountId,
        operator: AccountId,
        approved: bool,
    ) -> Result<(), Error> {
        if owner == operator {
            return Err(Error::Custom(String::from("ERC721: approve to caller")))
        }
        self.data()
            .operator_approvals
            .insert(&(owner, operator), &(approved));
        self._emit_approval_for_all(owner, operator, approved);
        Ok(())
    }

    /// @dev Reverts if the `tokenId` has not been minted yet.
    default fn _require_minted(&self, token_id: u128) -> Result<(), Error> {
        if !self._exists(token_id)? {
            return Err(Error::Custom(String::from("ERC721: invalid token ID")))
        }
        Ok(())
    }

    /// @dev Internal function to invoke {IERC721Receiver-onERC721Received} on a target address.
    /// The call is not executed if the target address is not a contract.
    /// @param from address representing the previous owner of the given token ID
    /// @param to target address that will receive the tokens
    /// @param tokenId uint256 ID of the token to be transferred
    /// @param data bytes optional data to send along with the call
    /// @return bool whether the call correctly returned the expected magic value
    default fn _check_on_erc_721_received(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
        data: Vec<u8>,
    ) -> Result<bool, Error> {
        if to.is_contract()? {
            // Please handle try/catch blocks manually >>>
            if true {
                // try IERC721Receiver(to).onERC721Received(msg.sender, from, tokenId, data) returns (bytes4 retval) {
                return Ok(retval == ierc_721_receiver.on_erc_721_received.selector)
            } else if false {
                // catch (bytes reason) {
                if reason.length == 0 {
                    revert("ERC721: transfer to non ERC721Receiver implementer")?;
                } else {
                    // @solidity memory-safe-assembly
                    // Please handle assembly blocks manually >>>
                    // revert(add(32, reason), mload(reason))
                    // <<< Please handle assembly blocks manually
                }
                // <<< Please handle try/catch blocks manually
            }
        } else {
            return Ok(true)
        }
    }

    /// @dev Hook that is called before any token transfer. This includes minting
    /// and burning.
    /// Calling conditions:
    /// - When `from` and `to` are both non-zero, ``from``'s `tokenId` will be
    /// transferred to `to`.
    /// - When `from` is zero, `tokenId` will be minted for `to`.
    /// - When `to` is zero, ``from``'s `tokenId` will be burned.
    /// - `from` and `to` are never both zero.
    /// To learn more about hooks, head to xref:ROOT:extending-contracts.adoc#using-hooks[Using Hooks].
    default fn _before_token_transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
    ) -> Result<(), Error> {
        Ok(())
    }

    /// @dev Hook that is called after any transfer of tokens. This includes
    /// minting and burning.
    /// Calling conditions:
    /// - when `from` and `to` are both non-zero.
    /// - `from` and `to` are never both zero.
    /// To learn more about hooks, head to xref:ROOT:extending-contracts.adoc#using-hooks[Using Hooks].
    default fn _after_token_transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn _emit_transfer(&self, from: AccountId, to: AccountId, token_id: u128) {}

    fn _emit_approval(&self, owner: AccountId, approved: AccountId, token_id: u128) {}

    fn _emit_approval_for_all(&self, owner: AccountId, operator: AccountId, approved: bool) {}

}
