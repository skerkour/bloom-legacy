package service

/*
	logger := rz.FromCtx(ctx)
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("mutation.CreateGroup: Starting transaction", rz.Err(err))
		err = gqlerrors.New(groups.NewError(groups.ErrorCreatingGroup))
		return
	}

	params := groups.CreateGroupParams{
		Name:               input.Name,
		Description:        input.Description,
		EncryptedMasterKey: input.EncryptedMasterKey,
		MasterKeyNonce:     input.MasterKeyNonce,
	}
	newGroup, err := groups.CreateGroup(ctx, tx, currentUser, params)
	if err != nil {
		tx.Rollback()
		err = gqlerrors.New(err)
		return
	}

	// create customer profile
	_, err = billing.CreateCustomer(ctx, tx, currentUser, nil, &newGroup.ID)
	if err != nil {
		tx.Rollback()
		err = gqlerrors.New(err)
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("mutation.CreateGroup: Committing transaction", rz.Err(err))
		err = gqlerrors.New(groups.NewError(groups.ErrorCreatingGroup))
		return
	}
	*/
