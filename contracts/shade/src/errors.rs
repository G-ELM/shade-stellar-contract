//! Contract error codes.
//!
//! Soroban hard-caps every `#[contracterror]` enum at **50 cases**
//! (`VecM<_, 50>` for `ScSpecUdtErrorEnumV0`). Exceeding it makes the derive
//! macro panic at compile time with a confusing `LengthExceedsMax`.
//!
//! Errors are therefore split across several domain enums rather than one
//! monolithic `ContractError`. The numeric codes are globally partitioned into
//! non-overlapping ranges so a raw code observed on-chain maps back to exactly
//! one variant, regardless of which enum produced it:
//!
//! | Range     | Enum                |
//! |-----------|---------------------|
//! | 1–99      | [`ContractError`]   |
//! | 100–119   | [`GovernanceError`] |
//! | 120–139   | [`EventError`]      |
//! | 140–159   | [`MultiSigError`]   |
//! | 160–179   | [`EscrowError`]     |
//! | 200–239   | [`CampaignError`]   |
//! | 240–259   | [`StretchGoalError`]|

use soroban_sdk::contracterror;

/// Core payment-engine errors: initialization, auth, merchants, invoices,
/// subscriptions, fees, tokens and oracles. Codes 1–99.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ContractError {
    NotAuthorized = 1,
    AlreadyInitialized = 2,
    NotInitialized = 3,
    Reentrancy = 4,
    MerchantAlreadyRegistered = 5,
    MerchantNotFound = 6,
    InvalidAmount = 7,
    InvoiceNotFound = 8,
    ContractPaused = 9,
    ContractNotPaused = 10,
    MerchantKeyNotFound = 11,
    TokenNotAccepted = 12,
    NonceAlreadyUsed = 14,
    InvalidInvoiceStatus = 16,
    RefundPeriodExpired = 17,
    WasmHashNotSet = 18,
    MerchantAccountNotSet = 20,
    InvalidInterval = 21,
    PlanNotFound = 22,
    PlanNotActive = 23,
    SubscriptionNotFound = 24,
    SubscriptionNotActive = 25,
    ChargeTooEarly = 26,
    InvoiceExpired = 27,
    InvoiceNotPaid = 28,
    PayerNotAvailable = 29,
    InsufficientBalance = 30,
    MerchantNotActive = 32,
    InvalidDescription = 33,
    OracleNotConfigured = 34,
    OraclePriceUnavailable = 35,
    TokenNotAcceptedByMerchant = 41,
    FeeUpdateTooEarly = 42,
    NoPendingFeeUpdate = 43,
    InvalidSwapPath = 44,
    InvalidSlippage = 45,
    /// The referenced escrow record does not exist.
    EscrowNotFound = 46,
    /// The escrow is not in a state that permits this operation.
    InvalidEscrowStatus = 47,
    /// Generic "record does not exist" for lookups without a dedicated code.
    NotFound = 48,
    /// This external-chain deposit has already been credited (idempotency guard).
    BridgeDepositProcessed = 49,
}

/// DAO governance errors. Codes 100–119.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum GovernanceError {
    /// Caller is not a registered governance council member.
    NotGovMember = 100,
    /// Governance voting parameters have not been configured.
    GovNotConfigured = 101,
    /// Supplied governance config is invalid (zero period or quorum > 100%).
    InvalidGovConfig = 102,
    /// No proposal exists for the supplied id.
    ProposalNotFound = 103,
    /// The proposal is no longer open (already executed or defeated).
    ProposalNotActive = 104,
    /// The voting window for this proposal has closed.
    VotingClosed = 105,
    /// The voting window is still open; the proposal cannot be finalized yet.
    VotingStillOpen = 106,
    /// This member has already voted on the proposal.
    AlreadyVoted = 107,
}

/// Ticketing / event errors. Codes 120–139.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum EventError {
    EventNotFound = 120,
    EventSoldOut = 121,
    InvalidCapacity = 122,
    InvalidEventDate = 123,
    InvalidRoyaltyBps = 124,
    TicketNotFound = 125,
    NotTicketOwner = 126,
    InvalidResalePrice = 127,
    TicketNotListed = 128,
    TicketAlreadyListed = 129,
}

/// Multi-sig massive-withdrawal errors. Codes 140–159.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum MultiSigError {
    /// The withdrawal amount is below the configured threshold; no multi-sig needed.
    BelowMultiSigThreshold = 140,
    /// No signers have been registered for multi-sig.
    MultiSigSignersNotSet = 141,
    /// The quorum value must be > 0 and <= the number of registered signers.
    InvalidQuorum = 142,
    /// The caller is not in the registered signer list.
    NotASigner = 143,
    /// This signer has already approved the given proposal.
    AlreadyApproved = 144,
    /// The referenced withdrawal proposal does not exist.
    ProposalNotFound = 145,
    /// The proposal is not in the Pending state and cannot be acted on.
    ProposalNotPending = 146,
    /// The proposal has not yet collected enough approvals for execution.
    QuorumNotReached = 147,
    /// Only the original proposer (merchant) may cancel their own proposal.
    NotProposer = 148,
    /// The multi-sig threshold has not been configured for this token.
    ThresholdNotSet = 149,
}

/// Escrow expired-refund errors. Codes 160–179.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum EscrowError {
    /// The escrow invoice has not yet reached its expiration timestamp.
    EscrowNotExpired = 160,
    /// The escrow invoice has already been fully refunded.
    EscrowAlreadyRefunded = 161,
}

/// Crowdfunding campaign errors: campaigns, categories, tags, backer reward
/// tiers, perks, comments and hard-cap voting. Codes 200–239.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum CampaignError {
    /// Referenced campaign does not exist.
    CampaignNotFound = 200,
    /// Referenced campaign category does not exist.
    CampaignCategoryNotFound = 201,
    /// A category with the supplied name has already been registered.
    CampaignCategoryAlreadyExists = 202,
    /// Referenced campaign category exists but is not active.
    CampaignCategoryInactive = 203,
    /// Referenced campaign tag does not exist.
    CampaignTagNotFound = 204,
    /// A tag with the supplied name has already been registered.
    CampaignTagAlreadyExists = 205,
    /// Campaign goal_amount must be positive.
    InvalidCampaignGoal = 206,
    /// Campaign deadline must be in the future.
    InvalidCampaignDeadline = 207,
    /// Operation referred to a campaign that has been deactivated.
    CampaignInactive = 208,
    /// The caller is not the merchant that owns the campaign.
    NotCampaignMerchant = 209,
    /// The campaign's deadline has passed; no further contributions accepted.
    CampaignExpired = 210,
    /// The campaign has already been closed out.
    CampaignEnded = 211,
    /// The campaign is not currently active.
    CampaignNotActive = 212,
    /// Referenced affiliate is not registered on this campaign.
    AffiliateNotFound = 213,
    /// Referenced reward tier does not exist on this campaign.
    InvalidRewardTier = 214,
    /// Pledge amount is below the selected tier's minimum.
    PledgeBelowTierMinimum = 215,
    /// The selected reward tier has reached its backer limit.
    RewardTierAtCapacity = 216,
    /// This backer's reward has already been marked fulfilled.
    BackerRewardAlreadyFulfilled = 217,
    /// The reward must be fulfilled before this operation.
    BackerRewardNotFulfilled = 218,
    /// The caller has not backed this campaign.
    NotBacker = 219,
    /// Referenced perk does not exist on the tier.
    PerkNotFound = 220,
    /// This perk has already been claimed by the backer.
    PerkAlreadyClaimed = 221,
    /// Reward tiers must be supplied in ascending minimum-pledge order.
    InvalidTierOrdering = 222,
    /// Generic NFT reward failure.
    NftError = 223,
    /// Referenced comment does not exist.
    CommentNotFound = 224,
    /// Comment body may not be empty.
    EmptyComment = 225,
    /// The comment is not in a state that permits this operation.
    InvalidCommentStatus = 226,
    /// Referenced hard-cap vote does not exist.
    VotingNotFound = 227,
    /// The hard-cap vote is not currently open.
    VotingNotActive = 228,
    /// This address has already cast a hard-cap vote.
    AlreadyVoted = 229,
}

/// Stretch-goal milestone errors. Codes 240–259.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum StretchGoalError {
    /// No stretch goal exists for the supplied id.
    StretchGoalNotFound = 240,
    /// The goal has not been unlocked yet.
    StretchGoalNotUnlocked = 241,
    /// The goal has already left the `Pending` state.
    GoalAlreadyUnlocked = 242,
    /// This backer has already claimed the reward for this goal.
    RewardAlreadyClaimed = 243,
    /// No reward has been granted to this backer for this goal.
    RewardNotFound = 244,
    /// A reward has already been granted to this backer for this goal.
    RewardAlreadyGranted = 245,
    /// The goal's target must exceed the campaign's base funding goal.
    TargetBelowBaseGoal = 246,
    /// Stretch goal targets must be strictly increasing per campaign.
    TargetNotIncreasing = 247,
    /// The campaign has not raised enough to unlock this goal.
    TargetNotReached = 248,
    /// The goal was cancelled and can no longer be unlocked or claimed.
    GoalCancelled = 249,
    /// Only the campaign's owning merchant may perform this operation.
    NotGoalOwner = 250,
    /// The campaign already has the maximum number of stretch goals.
    TooManyStretchGoals = 251,
}
