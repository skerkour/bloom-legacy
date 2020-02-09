module gitlab.com/bloom42/bloom

go 1.13

require (
	github.com/99designs/gqlgen v0.10.3-0.20200124025457-f7a67722a6ba
	github.com/getsentry/sentry-go v0.5.1
	github.com/go-chi/chi v4.0.3+incompatible
	github.com/go-chi/cors v1.0.0
	github.com/golang-migrate/migrate/v4 v4.8.0
	github.com/google/uuid v1.1.1
	github.com/jmoiron/sqlx v1.2.0
	github.com/lib/pq v1.3.0
	github.com/spf13/cobra v0.0.5
	github.com/stripe/stripe-go v68.18.0+incompatible
	github.com/vektah/gqlparser v1.3.1
	gitlab.com/bloom42/bloom/common v0.0.0-20200209095800-c52d93f12536
	gitlab.com/bloom42/bloom/server v0.0.0-20200209095800-c52d93f12536
	gitlab.com/bloom42/libs/crypto42-go v0.0.0-20200118201250-b035ee487899
	gitlab.com/bloom42/libs/rz-go v1.3.0
	gitlab.com/bloom42/libs/sane-go v0.10.0
)
