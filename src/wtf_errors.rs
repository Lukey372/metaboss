#![allow(unused)]
use phf::phf_map;

pub static METADATA_ERROR: phf::Map<&'static str, &'static str> = phf_map! {
    "0" => "InstructionUnpackError: Failed to unpack instruction data",
    "1" => "InstructionPackError: Failed to pack instruction data",
    "2" => "NotRentExempt: Lamport balance below rent-exempt threshold",
    "3" => "AlreadyInitialized: Already initialized",
    "4" => "Uninitialized: Uninitialized",
    "5" => "InvalidMetadataKey:  Metadata's key must match seed of ['metadata', program id, mint] provided",
    "6" => "InvalidEditionKey: Edition's key must match seed of ['metadata', program id, name, 'edition'] provided",
    "7" => "UpdateAuthorityIncorrect: Update Authority given does not match",
    "8" => "UpdateAuthorityIsNotSigner: Update Authority needs to be signer to update metadata",
    "9" => "NotMintAuthority: You must be the mint authority and signer on this transaction",
    "A" => "InvalidMintAuthority: Mint authority provided does not match the authority on the mint",
    "B" => "NameTooLong: Name too long",
    "C" => "SymbolTooLong: Symbol too long",
    "D" => "UriTooLong: URI too long",
    "E" => "UpdateAuthorityMustBeEqualToMetadataAuthorityAndSigner: Update authority must be equivalent to the metadata's authority and also signer of this transaction",
    "F" => "MintMismatch: Mint given does not match mint on Metadata",
    "10" => "EditionsMustHaveExactlyOneToken: Editions must have exactly one token",
    "11" => "MaxEditionsMintedAlready: Maximum editions printed already",
    "12" => "TokenMintToFailed: Token mint to failed",
    "13" => "MasterRecordMismatch: The master edition record passed must match the master record on the edition given",
    "14" => "DestinationMintMismatch: The destination account does not have the right mint",
    "15" => "EditionAlreadyMinted: An edition can only mint one of its kind!",
    "16" => "PrintingMintDecimalsShouldBeZero: Printing mint decimals should be zero",
    "17" => "OneTimePrintingAuthorizationMintDecimalsShouldBeZero: OneTimePrintingAuthorization mint decimals should be zero",
    "18" => "EditionMintDecimalsShouldBeZero: EditionMintDecimalsShouldBeZero",
    "19" => "TokenBurnFailed: Token burn failed",
    "1A" => "TokenAccountOneTimeAuthMintMismatch: The One Time authorization mint does not match that on the token account!",
    "1B" => "DerivedKeyInvalid: Derived key invalid",
    "1C" => "PrintingMintMismatch: The Printing mint does not match that on the master edition!",
    "1D" => "OneTimePrintingAuthMintMismatch: The One Time Printing Auth mint does not match that on the master edition!",
    "1E" => "TokenAccountMintMismatch: The mint of the token account does not match the Printing mint!",
    "1F" => "TokenAccountMintMismatchV2: The mint of the token account does not match the master metadata mint!",
    "20" => "NotEnoughTokens: Not enough tokens to mint a limited edition",
    "21" => "PrintingMintAuthorizationAccountMismatch",
    "22" => "AuthorizationTokenAccountOwnerMismatch: The authorization token account has a different owner than the update authority for the master edition!",
    "23" => "Disabled: This feature is currently disabled.",
    "24" => "CreatorsTooLong: Creators list too long",
    "25" => "CreatorsMustBeAtleastOne: Creators must be at least one if set",
    "26" => "MustBeOneOfCreators: If using a creators array, you must be one of the creators listed",
    "27" => "NoCreatorsPresentOnMetadata: This metadata does not have creators",
    "28" => "CreatorNotFound: This creator address was not found",
    "29" => "InvalidBasisPoints: Basis points cannot be more than 10000",
    "2A" => "PrimarySaleCanOnlyBeFlippedToTrue: Primary sale can only be flipped to true and is immutable",
    "2B" => "OwnerMismatch: Owner does not match that on the account given",
    "2C" => "NoBalanceInAccountForAuthorization: This account has no tokens to be used for authorization",
    "2D" => "ShareTotalMustBe100: Share total must equal 100 for creator array",
    "2E" => "ReservationExists: This reservation list already exists!",
    "2F" => "ReservationDoesNotExist: This reservation list does not exist!",
    "30" => "ReservationNotSet: This reservation list exists but was never set with reservations",
    "31" => "ReservationAlreadyMade: This reservation list has already been set!",
    "32" => "BeyondMaxAddressSize: Provided more addresses than max allowed in single reservation",
    "33" => "NumericalOverflowError: NumericalOverflowError",
    "34" => "ReservationBreachesMaximumSupply: This reservation would go beyond the maximum supply of the master edition!",
    "35" => "AddressNotInReservation: Address not in reservation!",
    "36" => "CannotVerifyAnotherCreator: You cannot unilaterally verify another creator, they must sign",
    "37" => "CannotUnverifyAnotherCreator: You cannot unilaterally unverify another creator",
    "38" => "SpotMismatch: In initial reservation setting, spots remaining should equal total spots",
    "39" => "IncorrectOwner: Incorrect account owner",
    "3A" => "PrintingWouldBreachMaximumSupply: printing these tokens would breach the maximum supply limit of the master edition",
    "3B" => "DataIsImmutable: Data is immutable",
    "3C" => "DuplicateCreatorAddress: No duplicate creator addresses",
    "3D" => "ReservationSpotsRemainingShouldMatchTotalSpotsAtStart: Reservation spots remaining should match total spots when first being created",
    "3E" => "InvalidTokenProgram: Invalid token program",
    "3F" => "DataTypeMismatch: Data type mismatch",
    "40" => "BeyondAlottedAddressSize: Beyond alotted address size in reservation!",
    "41" => "ReservationNotComplete: The reservation has only been partially alotted",
    "42" => "TriedToReplaceAnExistingReservation: You cannot splice over an existing reservation!",
    "43" => "InvalidOperation: Invalid operation",
    "44" => "InvalidOwner: Invalid Owner",
    "45" => "PrintingMintSupplyMustBeZeroForConversion: Printing mint supply must be zero for conversion",
    "46" => "OneTimeAuthMintSupplyMustBeZeroForConversion: One Time Auth mint supply must be zero for conversion",
    "47" => "InvalidEditionIndex: You tried to insert one edition too many into an edition mark pda",
    "48" => "ReservationArrayShouldBeSizeOne: In the legacy system the reservation needs to be of size one for cpu limit reasons",
    "49" => "IsMutableCanOnlyBeFlippedToFalse: Is Mutable can only be flipped to false",
    "4A" => "CollectionCannotBeVerifiedInThisInstruction: Cannont Verify Collection in this Instruction",
    "4B" => "Removed: This instruction was deprecated in a previous release and is now removed",
    "4C" => "MustBeBurned: This token use method is burn and there are no remaining uses, it must be burned",
    "4D" => "InvalidUseMethod: This use method is invalid",
    "4E" => "CannotChangeUseMethodAfterFirstUse: Cannot Change Use Method after the first use",
    "4F" => "CannotChangeUsesAfterFirstUse: Cannot Change Remaining or Available uses after the first use",
    "50" => "CollectionNotFound: Collection Not Found on Metadata",
    "51" => "InvalidCollectionUpdateAuthority: Collection Update Authority is invalid",
    "52" => "CollectionMustBeAUniqueMasterEdition: Collection Must Be a Unique Master Edition v2",
    "53" => "UseAuthorityRecordAlreadyExists: The Use Authority Record Already Exists, to modify it Revoke, then Approve",
    "54" => "UseAuthorityRecordAlreadyRevoked: The Use Authority Record is empty or already revoked",
    "55" => "Unusable: This token has no uses",
    "56" => "NotEnoughUses: There are not enough Uses left on this token.",
    "57" => "CollectionAuthorityRecordAlreadyExists: This Collection Authority Record Already Exists.",
    "58" => "CollectionAuthorityDoesNotExist: This Collection Authority Record Does Not Exist.",
    "59" => "InvalidUseAuthorityRecord: This Use Authority Record is invalid.",
    "5A" => "InvalidCollectionAuthorityRecord: This Collection Authority Record is invalid.",
    "5B" => "InvalidFreezeAuthority: Metadata does not match the freeze authority on the mint",
    "5C" => "InvalidDelegate: All tokens in this account have not been delegated to this user.",
    "5D" => "CannotAdjustVerifiedCreator: Creator can not be adjusted once they are verified.",
    "5E" => "CannotRemoveVerifiedCreator: Verified creators cannot be removed.",
    "5F" => "CannotWipeVerifiedCreators: Can not wipe verified creators.",
    "60" => "NotAllowedToChangeSellerFeeBasisPoints: Not allowed to change seller fee basis points.",
    "61" => "EditionOverrideCannotBeZero: Edition override cannot be zero",
    "62" => "InvalidUser: Invalid User",
    "63" => "RevokeCollectionAuthoritySignerIncorrect: Revoke Collection Authority signer is incorrect",
    "64" => "TokenCloseFailed: Token close failed",
    "65" => "UnsizedCollection: Can't use this function on unsized collection",
    "66" => "SizedCollection: Can't use this function on a sized collection",
    "67" => "MissingCollectionMetadata",
    "68" => "NotAMemberOfCollection: This NFT is not a member of the specified collection.",
    "69" => "NotVerifiedMemberOfCollection: This NFT is not a verified member of the specified collection.",
    "6A" => "NotACollectionParent: This NFT is not a collection parent NFT.",
    "6B" => "CouldNotDetermineTokenStandard: Could not determine a TokenStandard type.",
    "6C" => "MissingEditionAccount: This mint account has an edition but none was provided.",
    "6D" => "NotAMasterEdition: This edition is not a Master Edition",
    "6E" => "MasterEditionHasPrints: This Master Edition has existing prints",
    "6F" => "BorshDeserializationError: Borsh Deserialization Error",
    "70" => "CannotUpdateVerifiedCollection: Cannot update a verified colleciton in this command",
    "71" => "CollectionMasterEditionAccountInvalid: Edition account doesnt match collection ",
    "72" => "AlreadyVerified: Item is already verified.",
    "73" => "AlreadyUnverified: Item is already unverified.",
    "74" => "NotAPrintEdition: This edition is not a Print Edition",
    "75" => "InvalidMasterEdition: Invalid Master Edition",
    "76" => "InvalidPrintEdition: Invalid Print Edition",
    "77" => "InvalidEditionMarker: Invalid Edition Marker",
    "78" => "ReservationListDeprecated: Reservation List is Deprecated",
    "79" => "PrintEditionDoesNotMatchMasterEdition: Print Edition does not match Master Edition",
    "7A" => "EditionNumberGreaterThanMaxSupply: Edition Number greater than max supply",
};

pub static AUCTIONEER_ERROR: phf::Map<&'static str, &'static str> = phf_map! {
    "1770" => "BumpSeedNotInHashMap: Bump seed not in hash map",
    "1771" => "AuctionNotStarted: Auction has not started yet",
    "1772" => "AuctionEnded: Auction has ended",
    "1773" => "AuctionActive: Auction has not ended yet",
    "1774" => "BidTooLow: The bid was lower than the highest bid",
    "1775" => "SignerNotAuth: The signer must be the Auction House authority",
    "1776" => "NotHighestBidder: Execute Sale must be run on the highest bidder",
    "1777" => "BelowReservePrice: The bid price must be greater than the reserve price",
    "1778" => "BelowBidIncrement: The bid must match the highest bid plus the minimum bid increment",
    "1779" => "CannotCancelHighestBid: The highest bidder is not allowed to cancel",
};

pub static AUCTION_HOUSE_ERROR: phf::Map<&'static str, &'static str> = phf_map! {
    "1770" => "PublicKeyMismatch: PublicKeyMismatch",
    "1771" => "InvalidMintAuthority: InvalidMintAuthority",
    "1772" => "UninitializedAccount: UninitializedAccount",
    "1773" => "IncorrectOwner: IncorrectOwner",
    "1774" => "PublicKeysShouldBeUnique: PublicKeysShouldBeUnique",
    "1775" => "StatementFalse: StatementFalse",
    "1776" => "NotRentExempt: NotRentExempt",
    "1777" => "NumericalOverflow: NumericalOverflow",
    "1778" => "ExpectedSolAccount: Expected a sol account but got an spl token account instead",
    "1779" => "CannotExchangeSOLForSol: Cannot exchange sol for sol",
    "177A" => "SOLWalletMustSign: If paying with sol, sol wallet must be signer",
    "177B" => "CannotTakeThisActionWithoutAuctionHouseSignOff: Cannot take this action without auction house signing too",
    "177C" => "NoPayerPresent: No payer present on this txn",
    "177D" => "DerivedKeyInvalid: Derived key invalid",
    "177E" => "MetadataDoesntExist: Metadata doesn't exist",
    "177F" => "InvalidTokenAmount: Invalid token amount",
    "1780" => "BothPartiesNeedToAgreeToSale: Both parties need to agree to this sale",
    "1781" => "CannotMatchFreeSalesWithoutAuctionHouseOrSellerSignoff: Cannot match free sales unless the auction house or seller signs off",
    "1782" => "SaleRequiresSigner: This sale requires a signer",
    "1783" => "OldSellerNotInitialized: Old seller not initialized",
    "1784" => "SellerATACannotHaveDelegate: Seller ata cannot have a delegate set",
    "1785" => "BuyerATACannotHaveDelegate: Buyer ata cannot have a delegate set",
    "1786" => "NoValidSignerPresent: No valid signer present",
    "1787" => "InvalidBasisPoints: BP must be less than or equal to 10000",
    "1788" => "TradeStateDoesntExist: The trade state account does not exist",
    "1789" => "TradeStateIsNotEmpty: The trade state is not empty",
    "178A" => "ReceiptIsEmpty: The receipt is empty",
    "178B" => "InstructionMismatch: The instruction does not match",
    "178C" => "InvalidAuctioneer: Invalid Auctioneer for this Auction House instance.",
    "178D" => "MissingAuctioneerScope: The Auctioneer does not have the correct scope for this action.",
    "178E" => "MustUseAuctioneerHandler: Must use auctioneer handler.",
    "178F" => "NoAuctioneerProgramSet: No Auctioneer program set.",
    "1790" => "TooManyScopes: Too many scopes.",
    "1791" => "AuctionHouseNotDelegated: Auction House not delegated.",
    "1792" => "BumpSeedNotInHashMap: Bump seed not in hash map.",
    "1793" => "EscrowUnderRentExemption: The instruction would drain the escrow below rent exemption threshold",
    "1794" => "InvalidSeedsOrAuctionHouseNotDelegated: Invalid seeds or Auction House not delegated",
    "1795" => "BuyerTradeStateNotValid: The buyer trade state was unable to be initialized.",
    "1796" => "MissingElementForPartialOrder: Partial order size and price must both be provided in a partial buy.",
    "1797" => "NotEnoughTokensAvailableForPurchase: Amount of tokens available for purchase is less than the partial order amount.",
    "1798" => "PartialPriceMismatch: Calculated partial price does not not partial price that was provided.",
    "1799" => "AuctionHouseAlreadyDelegated: Auction House already delegated.",
    "179A" => "AuctioneerAuthorityMismatch: Auctioneer Authority Mismatch",
    "179B" => "InsufficientFunds: Insufficient funds in escrow account to purchase.",
};

pub static CANDY_ERROR: phf::Map<&'static str, &'static str> = phf_map! {
    "1770" => "IncorrectOwner: Account does not have correct owner!",
    "1771" => "Uninitialized: Account is not initialized!",
    "1772" => "MintMismatch: Mint Mismatch!",
    "1773" => "IndexGreaterThanLength: Index greater than length!",
    "1774" => "NumericalOverflowError: Numerical overflow error!",
    "1775" => "TooManyCreators: Can only provide up to 4 creators to candy machine (because candy machine is one)!",
    "1776" => "UuidMustBeExactly6Length: Uuid must be exactly of 6 length",
    "1777" => "NotEnoughTokens: Not enough tokens to pay for this minting",
    "1778" => "NotEnoughSOL: Not enough SOL to pay for this minting",
    "1779" => "TokenTransferFailed: Token transfer failed",
    "177A" => "CandyMachineEmpty: Candy machine is empty!",
    "177B" => "CandyMachineNotLive: Candy machine is not live!",
    "177C" => "HiddenSettingsConfigsDoNotHaveConfigLines: Configs that are using hidden uris do not have config lines, they have a single hash representing hashed order",
    "177D" => "CannotChangeNumberOfLines: Cannot change number of lines unless is a hidden config",
    "177E" => "DerivedKeyInvalid: Derived key invalid",
    "177F" => "PublicKeyMismatch: Public key mismatch",
    "1780" => "NoWhitelistToken: No whitelist token present",
    "1781" => "TokenBurnFailed: Token burn failed",
    "1782" => "GatewayAppMissing: Missing gateway app when required",
    "1783" => "GatewayTokenMissing: Missing gateway token when required",
    "1784" => "GatewayTokenExpireTimeInvalid: Invalid gateway token expire time",
    "1785" => "NetworkExpireFeatureMissing: Missing gateway network expire feature when required",
    "1786" => "CannotFindUsableConfigLine: Unable to find an unused config line near your random number index",
    "1787" => "InvalidString: Invalid string",
    "1788" => "SuspiciousTransaction: Suspicious transaction detected",
    "1789" => "CannotSwitchToHiddenSettings: Cannot Switch to Hidden Settings after items available is greater than 0",
    "178A" => "IncorrectSlotHashesPubkey: Incorrect SlotHashes PubKey",
    "178B" => "IncorrectCollectionAuthority: Incorrect collection NFT authority",
    "178C" => "MismatchedCollectionPDA: Collection PDA address is invalid",
    "178D" => "MismatchedCollectionMint: Provided mint account doesn't match collection PDA mint",
    "178E" => "SlotHashesEmpty: Slot hashes Sysvar is empty",
    "178F" => "MetadataAccountMustBeEmpty: The metadata account has data in it, and this must be empty to mint a new NFT",
    "1790" => "MissingSetCollectionDuringMint: Missing set collection during mint IX for Candy Machine with collection set",
    "1791" => "NoChangingCollectionDuringMint: Can't change collection settings after items have begun to be minted",
    "1792" => "CandyCollectionRequiresRetainAuthority: Retain authority must be true for Candy Machines with a collection set",
    "1793" => "GatewayProgramError: Error within Gateway program",
    "1794" => "NoChangingFreezeDuringMint",
    "1795" => "NoChangingAuthorityWithCollection: Can't change authority while collection is enabled. Disable collection first.",
    "1796" => "NoChangingTokenWithFreeze: Can't change token while freeze is enabled. Disable freeze first.",
    "1797" => "InvalidThawNft: Cannot thaw NFT unless all NFTs are minted or Candy Machine authority enables thawing",
    "1798" => "IncorrectRemainingAccountsLen: The number of remaining accounts passed in doesn't match the Candy Machine settings",
    "1799" => "MissingFreezeAta: FreezePDA ATA needs to be passed in if token mint is enabled.",
    "179A" => "IncorrectFreezeAta: Incorrect freeze ATA address.",
    "179B" => "FreezePDAMismatch: FreezePDA doesn't belong to this Candy Machine.",
    "179C" => "EnteredFreezeIsMoreThanMaxFreeze: Freeze time can't be longer than MAX_FREEZE_TIME.",
    "179D" => "NoWithdrawWithFreeze: Can't withdraw Candy Machine while freeze is active. Disable freeze first.",
    "179E" => "NoWithdrawWithFrozenFunds",
    "179F" => "MissingRemoveFreezeTokenAccounts: Missing required remaining accounts for remove_freeze with token mint.",
    "17A0" => "InvalidFreezeWithdrawTokenAddress: Can't withdraw SPL Token from freeze PDA into itself",
    "17A1" => "NoUnlockWithNFTsStillFrozen: Can't unlock funds while NFTs are still frozen. Run thaw on all NFTs first.",
    "17A2" => "SizedCollectionMetadataMustBeMutable: Setting a sized collection requires the collection metadata to be mutable.",
};

pub static ANCHOR_ERROR: phf::Map<&'static str, &'static str> = phf_map! {
    "64" => "InstructionMissing: 8 byte instruction identifier not provided",
    "65" => "InstructionFallbackNotFound: Fallback functions are not supported",
    "66" => "InstructionDidNotDeserialize: The program could not deserialize the given instruction",
    "67" => "InstructionDidNotSerialize: The program could not serialize the given instruction",
    "3E8" => "IdlInstructionStub: The program was compiled without idl instructions",
    "3E9" => "IdlInstructionInvalidProgram: Invalid program given to the IDL instruction",
    "7D0" => "ConstraintMut: A mut constraint was violated",
    "7D1" => "ConstraintHasOne: A has one constraint was violated",
    "7D2" => "ConstraintSigner: A signer constraint was violated",
    "7D3" => "ConstraintRaw: A raw constraint was violated",
    "7D4" => "ConstraintOwner: An owner constraint was violated",
    "7D5" => "ConstraintRentExempt: A rent exemption constraint was violated",
    "7D6" => "ConstraintSeeds: A seeds constraint was violated",
    "7D7" => "ConstraintExecutable: An executable constraint was violated",
    "7D8" => "ConstraintState: A state constraint was violated",
    "7D9" => "ConstraintAssociated: An associated constraint was violated",
    "7DA" => "ConstraintAssociatedInit: An associated init constraint was violated",
    "7DB" => "ConstraintClose: A close constraint was violated",
    "7DC" => "ConstraintAddress: An address constraint was violated",
    "7DD" => "ConstraintZero: Expected zero account discriminant",
    "7DE" => "ConstraintTokenMint: A token mint constraint was violated",
    "7DF" => "ConstraintTokenOwner: A token owner constraint was violated",
    "7E0" => "ConstraintMintMintAuthority: A mint mint authority constraint was violated",
    "7E1" => "ConstraintMintFreezeAuthority: A mint freeze authority constraint was violated",
    "7E2" => "ConstraintMintDecimals: A mint decimals constraint was violated",
    "7E3" => "ConstraintSpace: A space constraint was violated",
    "9C4" => "RequireViolated: A require expression was violated",
    "9C5" => "RequireEqViolated: A require_eq expression was violated",
    "9C6" => "RequireKeysEqViolated: A require_keys_eq expression was violated",
    "9C7" => "RequireNeqViolated: A require_neq expression was violated",
    "9C8" => "RequireKeysNeqViolated: A require_keys_neq expression was violated",
    "9C9" => "RequireGtViolated: A require_gt expression was violated",
    "9CA" => "RequireGteViolated: A require_gte expression was violated",
    "BB8" => "AccountDiscriminatorAlreadySet: The account discriminator was already set on this account",
    "BB9" => "AccountDiscriminatorNotFound: No 8 byte discriminator was found on the account",
    "BBA" => "AccountDiscriminatorMismatch: 8 byte discriminator did not match what was expected",
    "BBB" => "AccountDidNotDeserialize: Failed to deserialize the account",
    "BBC" => "AccountDidNotSerialize: Failed to serialize the account",
    "BBD" => "AccountNotEnoughKeys: Not enough account keys given to the instruction",
    "BBE" => "AccountNotMutable: The given account is not mutable",
    "BBF" => "AccountOwnedByWrongProgram: The given account is owned by a different program than expected",
    "BC0" => "InvalidProgramId: Program ID was not as expected",
    "BC1" => "InvalidProgramExecutable: Program account is not executable",
    "BC2" => "AccountNotSigner: The given account did not sign",
    "BC3" => "AccountNotSystemOwned: The given account is not owned by the system program",
    "BC4" => "AccountNotInitialized: The program expected this account to be already initialized",
    "BC5" => "AccountNotProgramData: The given account is not a program data account",
    "BC6" => "AccountNotAssociatedTokenAccount: The given account is not the associated token account",
    "BC7" => "AccountSysvarMismatch: The given public key does not match the required sysvar",
    "BC8" => "AccountReallocExceedsLimit: The account reallocation exceeds the MAX_PERMITTED_DATA_INCREASE limit",
    "BC9" => "AccountDuplicateReallocs: The account was duplicated for more than one reallocation",
    "FA0" => "StateInvalidAddress: The given state account does not have the correct address",
    "1004" => "DeclaredProgramIdMismatch: The declared program id does not match the actual program id",
    "1388" => "Deprecated: The API being used is deprecated and should no longer be used",
};
