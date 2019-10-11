<template>
  <div>
    <v-container fluid grid-list-lg>
      <v-layout align-center justify-center>
        <v-flex xs12 sm6>
          <v-text-field placeholder="Take a note..." solo @click="openNoteDialog" readonly/>
        </v-flex>
      </v-layout>
      <v-layout row wrap justify-left class="mt-1">
        <v-flex v-for="note in notes" :key="note.id" xs12 sm4 md3>
          <blm-notes-note
            :note="note"
            @archive="noteArchived"
            @remove="noteRemoved"
            @update="noteUpdated"
            @delete="noteDeleted"
          />
        </v-flex>
    </v-layout>
    </v-container>

  <blm-notes-dialog-note
    :visible="noteDialog"
    @close="noteDialogClosed"
    @create="noteCreated"
    @update="noteUpdated"
  />
  </div>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import { Native, Message } from '@/native';
import Note from '../components/Note.vue';
import NoteDialog from '../components/NoteDialog.vue';
import { Note as NoteModel, GuiNotes } from '@/native/messages/notes';

const { log } = require('@bloom42/astro');

@Component({
  components: {
    'blm-notes-dialog-note': NoteDialog,
    'blm-notes-note': Note,
  },
})
export default class Index extends Vue {
  // props
  // data
  error = '';
  isLoading = false;
  notes: NoteModel[] = [];
  noteDialog = false;

  // computed
  // lifecycle
  async created() {
    const message: Message = {
      type: 'notes.gui.list_notes',
      data: {},
    };
    try {
      const res = await Native.call(message);
      this.notes = (res.data as GuiNotes).notes;
    } catch (err) {
      log.error(err);
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

  noteCreated(note: NoteModel) {
    this.notes = [note, ...this.notes];
  }

  noteUpdated(updatedNote: NoteModel) {
    const pos = this.notes.map((note: NoteModel) => note.id).indexOf(updatedNote.id);
    this.notes.splice(pos, 1);
    this.notes = [updatedNote, ...this.notes];
  }

  noteArchived(archivedNote: NoteModel) {
    this.notes = this.notes.filter((note: NoteModel) => note.id !== archivedNote.id);
  }

  noteRemoved(removedNote: NoteModel) {
    this.notes = this.notes.filter((note: NoteModel) => note.id !== removedNote.id);
  }

  noteDeleted(deletedNote: NoteModel) {
    this.notes = this.notes.filter((note: NoteModel) => note.id !== deletedNote.id);
  }
}
</script>


<style lang="scss" scoped>
</style>
