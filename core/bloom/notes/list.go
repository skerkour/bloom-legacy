package notes

type ListNotesParams struct{}

func ListNotes(_ ListNotesParams) ([]Note, error) {
	return []Note{}, nil
}
