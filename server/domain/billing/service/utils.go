package service

import (
	"gitlab.com/bloom42/bloom/common/consts"
)

func allowedStorageForProduct(product string) int64 {
	if product == consts.BILLING_PRODUCT_ULTRA {
		return 100000000000000 // 1TB
	} else if product == consts.BILLING_PRODUCT_LITE {
		return 100000000000 // 100GB
	} else if product == consts.BILLING_PRODUCT_PRO {
		return 400000000000 // 400GB
	} else { // FREE
		return 100000000 // 100MB
	}
}
