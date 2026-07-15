/**
 * Maps Plaid `institution_id`s (stable across all Plaid environments) to Iconify
 * icon names. 
 */
const INSTITUTION_LOGOS: Record<string, string> = {
  ins_56: "selfhst:chase", // Chase
  ins_127989: "selfhst:bank-of-america", // Bank of America
  ins_127991: "selfhst:wells-fargo", // Wells Fargo
  ins_5: "selfhst:citibank", // Citibank
  ins_128026: "selfhst:capital-one", // Capital One
  ins_10: "selfhst:american-express", // American Express
  ins_33: "selfhst:discover-card", // Discover
  ins_50: "selfhst:charles-schwab", // Charles Schwab
  ins_12: "selfhst:fidelity", // Fidelity
  ins_29: "selfhst:usaa", // USAA
  ins_15: "selfhst:navy-federal-credit-union", // Navy Federal Credit Union
  ins_116295: "selfhst:sofi", // SoFi
  ins_54: "selfhst:robinhood", // Robinhood
  ins_31: "selfhst:paypal", // PayPal
};

const DEFAULT_INSTITUTION_ICON = "mdi:bank";

export function getInstitutionIcon(institutionId?: string | null): string {
  if (!institutionId) {
    return DEFAULT_INSTITUTION_ICON;
  }
  return INSTITUTION_LOGOS[institutionId] ?? DEFAULT_INSTITUTION_ICON;
}
