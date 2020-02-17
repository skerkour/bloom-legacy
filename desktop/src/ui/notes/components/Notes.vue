<template>
  <div>
    <v-container fluid grid-list-lg>
      <v-layout align-center justify-center v-if="!archive">
        <v-flex xs12 sm6>
          <v-text-field placeholder="Take a note..." solo @click="openNoteDialog" readonly/>
        </v-flex>
      </v-layout>
      <v-row justify="start">
        <v-col v-for="note in notes" :key="note.id" cols="12" sm="6" md="4" lg="3">
          <blm-notes-note
            :note="note"
            @archived="noteArchived"
            @unarchived="noteUnarchived"
            @updated="noteUpdated"
            @deleted="noteDeleted"
          />
        </v-col>
    </v-row>
    </v-container>

  <blm-notes-dialog-note
    :visible="noteDialog"
    @closed="noteDialogClosed"
    @created="noteCreated"
  />
  </div>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import BlmNote from './Note.vue';
import NoteDialog from './NoteDialog.vue';
import core from '@/core';
import { Note, Notes, Method } from '@/core/notes';


const { log } = require('@bloom42/astro');

@Component({
  components: {
    'blm-notes-dialog-note': NoteDialog,
    'blm-notes-note': BlmNote,
  },
})
export default class Index extends Vue {
  // props
  @Prop({ type: Boolean, default: false }) archive!: boolean;

  // data
  error = '';
  isLoading = false;
  notes: Note[] = [];
  noteDialog = false;

  // computed
  // lifecycle
  async created() {
    if (this.archive) {
      this.fetchArchive();
    } else {
      this.fetchNotes();
    }
  }

  async fetchNotes() {
    this.error = '';
    this.isLoading = true;

    try {
      const res = await core.call(Method.ListNotes, core.Empty);
      this.notes = (res as Notes).notes;
    } catch (err) {
      log.error(err);
    } finally {
      this.isLoading = false;
    }
  }

  async fetchArchive() {
    this.error = '';
    this.isLoading = true;

    try {
      const res = await core.call(Method.ListArchived, core.Empty);
      this.notes = (res as Notes).notes;
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
  }

  // watch
  // methods
  openNoteDialog() {
    this.noteDialog = true;
  }

  noteDialogClosed() {
    this.noteDialog = false;
  }

  noteCreated(note: Note) {
    this.notes = [note, ...this.notes];
  }

  noteUpdated(updatedNote: Note) {
    const pos = this.notes.map((note: Note) => note.id).indexOf(updatedNote.id);
    this.notes.splice(pos, 1);
    this.notes = [updatedNote, ...this.notes];
    // this.notes = this.notes.map((note: any) => {
    //   if (note.id === updatedNote.id) {
    //     return updatedNote;
    //   }
    //   return note;
    // });
  }

  noteArchived(archivedNote: Note) {
    this.notes = this.notes.filter((note: Note) => note.id !== archivedNote.id);
  }

  noteUnarchived(unarchivedNote: Note) {
    this.notes = this.notes.filter((note: Note) => note.id !== unarchivedNote.id);
  }

  noteDeleted(deletedNote: Note) {
    this.notes = this.notes.filter((note: Note) => note.id !== deletedNote.id);
  }
}
</script>


<style lang="scss" scoped>
</style>
