package users

import (
	"gitlab.com/bloom42/bloom/core/domain/kernel"
)

func VerifyRegistration(params VerifyRegistrationParams) (kernel.Empty, error) {
	// client := users.NewUsersProtobufClient("http://localhost:8000", &http.Client{})

	// rpcParams := users.VerifyRegistrationParams{
	// 	Code: params.Code,
	// 	Id:   params.ID,
	// }

	// _, err := client.VerifyRegistration(context.Background(), &rpcParams)
	// if err != nil {
	// 	return kernel.Empty{}, err
	// }

	return kernel.Empty{}, nil
}
