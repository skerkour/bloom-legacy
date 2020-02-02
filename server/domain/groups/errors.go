package groups

import (
	"gitlab.com/bloom42/bloom/server/errors"
)

type DomainError int

const (
	ErrorAcceptingInvitation DomainError = iota
	ErrorInvitationNotFound
	ErrorCancelingInvitation
	ErrorInvalidArgument
	ErrorCreatingGroup
	ErrorDecliningInvitation
	ErrorDeletingGroup
	ErrorGroupNotFound
	ErrorPermissionDenied
	ErrorInvitingUsers
	ErrorUsernamesNotFound
	ErrorUserAlreadyInGroup
	ErrorQuittingGroup
	ErrorAtLeastOneAdministratorShouldRemainsInGroup
	ErrorUpdatingGroup
	ErrorRemovingMembersFromGroup
)

func NewError(domainError DomainError) errors.Error {
	code := errors.Internal
	message := "Internal error. Please try again."

	switch domainError {
	case ErrorAcceptingInvitation:
		message = "Error joining group. Please try again."
	case ErrorInvitationNotFound:
		code = errors.NotFound
		message = "Invitation not found."
	case ErrorCancelingInvitation:
		message = "Error canceling invitaiton. Please try again."
	case ErrorInvalidArgument:
		code = errors.InvalidArgument
		message = "Invalid argument. Please fix before retrying."
	case ErrorCreatingGroup:
		message = "Error creating new group. Please try again."
	case ErrorDecliningInvitation:
		message = "Error declining invitaiton. Please try again."
	case ErrorDeletingGroup:
		message = "Error deleting group. Please try again."
	case ErrorGroupNotFound:
		code = errors.NotFound
		message = "Group not found."
	case ErrorPermissionDenied:
		code = errors.PermissionDenied
		message = "Permission denied."
	case ErrorInvitingUsers:
		message = "Error inviting people. Please try again."
	case ErrorUsernamesNotFound:
		code = errors.NotFound
		message = "Some usernames were not found. Please verify your invitees list and retry."
	case ErrorUserAlreadyInGroup:
		code = errors.AlreadyExists
		message = "At least one user is already in group or invited. Please remove it and retry."
	case ErrorQuittingGroup:
		message = "Error quitting group. Please try again."
	case ErrorAtLeastOneAdministratorShouldRemainsInGroup:
		code = errors.PermissionDenied
		message = "At least one administrator should remain in group."
	case ErrorUpdatingGroup:
		message = "Error updating group. Please try again."
	case ErrorRemovingMembersFromGroup:
		message = "Error removing members from group. Please try again."
	}

	return errors.New(code, message)
}

func NewErrorMessage(domainError DomainError, message string) errors.Error {
	err := NewError(domainError)
	err.Message = message
	return err
}
