package billing

import (
	"gitlab.com/bloom42/bloom/common/consts"
)

func GetAllowedStorageFromPlanTier(tier string) int64 {
	if tier == consts.BILLING_FREE_TIER {
		return 1000000000 // 1GB
	} else if tier == consts.BILLING_BASIC_TIER {
		return 100000000000 // 100GB
	} else if tier == consts.BILLING_PRO_TIER {
		return 400000000000 // 400GB
	} else { // ULTRA
		return 100000000000000 // 1TB
	}
}
