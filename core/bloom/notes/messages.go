package notes

type CreateNoteParams struct {
	Title string `json:"title"`
	Body  string `json:"body"`
	Color string `json:"color"`
}

type Notes struct {
	Notes []Note `json:"notes"`
}

type DeleteNoteParams struct {
	ID string `json:"id"`
}
