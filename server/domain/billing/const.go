package billing

const (
	PlanNameMaxLength = 15
	PlanNameMinLength = 3

	ProductFree  = "FREE"
	ProductLite  = "LITE"
	ProductPro   = "PRO"
	ProductUltra = "ULTRA"

	PlanPriceFree  = 0
	PlanPriceLite  = 800
	PlanPricePro   = 1600
	PlanPriceUltra = 3200

	StorageFree  = 100000000       // 100MB
	StorageLite  = 100000000000    // 100GB
	StoragePro   = 400000000000    // 400GB
	StorageUltra = 100000000000000 // 1TB
)
