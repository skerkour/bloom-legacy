package repository

/*
	queryUpdate := `UPDATE objects
				SET algorithm = $1, nonce = $2, encrypted_key = $3, encrypted_data = $4, updated_at_state = $5
				WHERE id = $6
			`
			_, err = tx.Exec(queryUpdate, object.Algorithm, object.Nonce, object.EncryptedKey,
				object.EncryptedData, object.UpdatedAtState, object.ID)
			if err != nil {
				logger.Error("sync.push: inserting object",
					rz.String("object.id", base64.StdEncoding.EncodeToString(object.ID)),
					rz.String("object.group_id", object.GroupID.String()),
				)
				return
			}
*/
