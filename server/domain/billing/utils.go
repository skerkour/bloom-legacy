package billing

import (
	"gitlab.com/bloom42/bloom/common/consts"
)

func GetAllowedStorageFromPlanProduct(product string) int64 {
	if product == consts.BILLING_PRODUCT_ULTRA {
		return 100000000000000 // 1TB
	} else if product == consts.BILLING_PRODUCT_BASIC {
		return 100000000000 // 100GB
	} else if product == consts.BILLING_PRODUCT_PRO {
		return 400000000000 // 400GB
	} else { // FREE
		return 1000000000 // 1GB
	}
}
