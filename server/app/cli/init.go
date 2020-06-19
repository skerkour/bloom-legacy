package cli

import (
	"context"
	"time"

	"github.com/golang-migrate/migrate/v4"
	"gitlab.com/bloom42/bloom/server/app/config"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	billingrepository "gitlab.com/bloom42/bloom/server/domain/billing/repository"
	"gitlab.com/bloom42/gobox/cli"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

var initCmd = &cli.Command{
	Use:     "init",
	Aliases: []string{"init"},
	Short:   "Init database and load default data",
	RunE: func(cmd *cli.Command, args []string) (err error) {
		ctx := context.Background()
		conf, err := config.Load(configFileFlag)
		if err != nil {
			return err
		}

		db, err := db.Connect(conf.Database.URL, conf.Database.PoolSize)
		if err != nil {
			return err
		}

		migrate, err := migrate.New(migrationsDirPath, conf.Database.URL)
		if err != nil {
			return err
		}

		err = migrate.Up()
		if err != nil {
			return err
		}

		log.Info("Migrations successfully done")

		billingRepo := billingrepository.NewBillingRepository()

		now := time.Now().UTC()
		planFree := billing.Plan{
			ID:          uuid.New(),
			CreatedAt:   now,
			UpdatedAt:   now,
			Name:        "Free",
			Description: "",
			StripeID:    "plan_TOCHANGE",
			Price:       billing.PlanPriceFree,
			Product:     billing.ProductFree,
			Storage:     billing.StorageFree,
		}
		planLite := billing.Plan{
			ID:          uuid.New(),
			CreatedAt:   now,
			UpdatedAt:   now,
			Name:        "Lite",
			Description: "",
			StripeID:    "plan_TOCHANGE1",
			Price:       billing.PlanPriceLite,
			Product:     billing.ProductLite,
			Storage:     billing.StorageLite,
		}
		planPro := billing.Plan{
			ID:          uuid.New(),
			CreatedAt:   now,
			UpdatedAt:   now,
			Name:        "Pro",
			Description: "",
			StripeID:    "plan_TOCHANGE2",
			Price:       billing.PlanPricePro,
			Product:     billing.ProductPro,
			Storage:     billing.StoragePro,
		}
		planUltra := billing.Plan{
			ID:          uuid.New(),
			CreatedAt:   now,
			UpdatedAt:   now,
			Name:        "Ultra",
			Description: "",
			StripeID:    "plan_TOCHANGE3",
			Price:       billing.PlanPriceUltra,
			Product:     billing.ProductUltra,
			Storage:     billing.StorageUltra,
		}

		tx, err := db.Begin(ctx)
		if err != nil {
			return
		}

		err = billingRepo.CreatePlan(ctx, tx, planFree)
		if err != nil {
			tx.Rollback()
			return
		}

		err = billingRepo.CreatePlan(ctx, tx, planLite)
		if err != nil {
			tx.Rollback()
			return
		}

		err = billingRepo.CreatePlan(ctx, tx, planPro)
		if err != nil {
			tx.Rollback()
			return
		}

		err = billingRepo.CreatePlan(ctx, tx, planUltra)
		if err != nil {
			tx.Rollback()
			return
		}

		err = tx.Commit()
		if err != nil {
			tx.Rollback()
			return
		}

		log.Info("Default data successfully loaded")
		return
	},
}
