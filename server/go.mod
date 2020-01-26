module gitlab.com/bloom42/bloom/server

go 1.13

require (
	github.com/go-chi/chi v4.0.3+incompatible
	github.com/go-chi/cors v1.0.0
	github.com/golang-migrate/migrate/v4 v4.8.0
	github.com/google/uuid v1.1.1
	github.com/jmoiron/sqlx v1.2.0
	github.com/lib/pq v1.3.0
	github.com/spf13/cobra v0.0.5
	github.com/spf13/pflag v1.0.5 // indirect
	github.com/stripe/stripe-go v68.13.0+incompatible
	github.com/twitchtv/twirp v5.10.1+incompatible
	gitlab.com/bloom42/bloom/common v0.0.0-20200124213701-398354050fc3
	gitlab.com/bloom42/libs/crypto42-go v0.0.0-20200118201250-b035ee487899
	gitlab.com/bloom42/libs/rz-go v1.3.0
	gitlab.com/bloom42/libs/sane-go v0.10.0
	golang.org/x/sys v0.0.0-20200122134326-e047566fdf82 // indirect
)
