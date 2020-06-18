package config

import (
	"net/mail"
)

const (
	// EnvDevelopment is the value expected for the development environment
	EnvDevelopment = "dev"
	// EnvStaging is the value expected for the staging environment
	EnvStaging = "staging"
	// EnvProduction is the value expected for the production environment
	EnvProduction = "production"

	// MigrationsDir is the directory where the migrations files are
	MigrationsDir = "db/migrations"
)

// DefaultEmailAddressFrom is the default email sender address
var DefaultEmailAddressFrom = mail.Address{Name: "Bloom", Address: "hello@bloom.sh"}
