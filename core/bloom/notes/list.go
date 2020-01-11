package notes

import (
	"gitlab.com/bloom42/bloom/core/bloom/notes/models"
)

type ListNotesParams struct{}

func ListNotes(_ ListNotesParams) ([]models.Note, error) {
	return []models.Note{}, nil
}
