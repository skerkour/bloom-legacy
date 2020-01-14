package notes

import (
	"github.com/google/uuid"
	"gitlab.com/bloom42/bloom/core/db"
	"time"
)

// let now = chrono::Utc::now();
// let note = db::Note {
// 	id: uuid::Uuid::new_v4().to_hyphenated().to_string(),
// 	created_at: now,
// 	updated_at: now,
// 	archived_at: None,
// 	title: input.title,
// 	body: input.body,
// 	color: input.color,
// 	is_pinned: false,
// };

// conn.execute(
// 	"INSERT INTO notes (id, created_at, updated_at, archived_at, title, body, color, is_pinned)
// 		VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
// 	params![
// 		&note.id,
// 		&note.created_at,
// 		&note.updated_at,
// 		&note.archived_at,
// 		&note.title,
// 		&note.body,
// 		&note.color,
// 		&note.is_pinned,
// 	],
// )?;

func CreateNote(params CreateNoteParams) (Note, error) {
	now := time.Now().UTC()
	uuid := uuid.New()
	note := Note{
		ID:         uuid.String(),
		CreatedAt:  now,
		UpdatedAt:  now,
		ArchivedAt: nil,
		Title:      params.Title,
		Body:       params.Body,
		Color:      params.Color,
		IsPinned:   false,
	}

	stmt, err := db.DB.Prepare(`INSERT INTO notes (id, created_at, updated_at, archived_at, title, body, color, is_pinned)
		VALUES (?, ?, ?, ?, ?, ?, ?, ?)`)
	if err != nil {
		return note, nil
	}
	defer stmt.Close()

	_, err = stmt.Exec(&note.ID, &note.CreatedAt, &note.UpdatedAt, &note.ArchivedAt, &note.Title, &note.Body, &note.Color, &note.IsPinned)

	return note, err
}
