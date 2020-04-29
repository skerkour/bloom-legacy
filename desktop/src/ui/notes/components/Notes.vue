<template>
    <!-- <v-container fluid grid-list-lg>
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
  /> -->
  <!-- <v-container> -->
    <v-layout fill-height>
      <v-col cols="4" class="pa-0">
        <v-toolbar elevation="0">
           <v-spacer />
          <v-tooltip bottom>
            <template v-slot:activator="{ on }">
              <v-btn icon v-on="on">
                <v-icon>mdi-plus</v-icon>
              </v-btn>
            </template>
            <span>New Note</span>
          </v-tooltip>
        </v-toolbar>

        <v-list-item-group
          @change="selectedNoteChanged"
          :value="currentNoteIndex"
          color="indigo">
          <v-list three-line class="overflow-y-auto pa-0">
            <v-list-item v-for="(note, index) in notes" :key="note.id" >

              <v-list-item-content class="text-left">
                <v-list-item-title>{{ note.title }}</v-list-item-title>
                <v-list-item-subtitle>{{ note.body }}</v-list-item-subtitle>
              </v-list-item-content>

              <v-divider v-if="index !== notes.length - 1" />
            </v-list-item>
          </v-list>
        </v-list-item-group>
      </v-col>

      <v-col cols="8" class="pa-0 blm-main-col">
        <v-toolbar elevation="0" v-if="currentNote">
          <v-text-field
            :value="currentNote.title"
            placeholder="Title"
            hide-details
          ></v-text-field>

          <v-menu>
            <template v-slot:activator="{ on }">
              <v-btn icon v-on="on">
                <v-icon>mdi-dots-vertical</v-icon>
              </v-btn>
            </template>
            <v-list class="text-left">
              <v-list-item>
                <v-list-item-icon>
                  <v-icon>mdi-pin</v-icon>
                </v-list-item-icon>
                <v-list-item-title>Pin</v-list-item-title>
              </v-list-item>
              <v-list-item>
                <v-list-item-icon>
                  <v-icon>mdi-package-down</v-icon>
                </v-list-item-icon>
                <v-list-item-title>Archive</v-list-item-title>
              </v-list-item>
              <v-list-item>
                <v-list-item-icon>
                  <v-icon>mdi-delete</v-icon>
                </v-list-item-icon>
                <v-list-item-title>Delete forever</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-menu>
        </v-toolbar>
        <div class="overflow-y-auto ps-2" v-if="currentNote">
          <v-textarea
            v-model="currentNote.body"
            placeholder="Take a note..."
            autofocus
            hide-details
            solo
            flat
            height="calc(100vh - 80px)"
          ></v-textarea>
        </div>
      </v-col>
    </v-layout>
  <!-- </v-container> -->

</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import BlmNote from './Note.vue';
import NoteDialog from './NoteDialog.vue';
import core from '@/core';
import { Note, Notes, Method } from '@/core/notes';
import { log } from '@/libs/rz';

@Component({
  components: {
    'blm-notes-dialog-note': NoteDialog,
    'blm-notes-note': BlmNote,
  },
})
export default class NotesIndex extends Vue {
  // props
  @Prop({ type: Boolean, default: false }) archive!: boolean;

  // data
  error = '';
  isLoading = false;
  notes: Note[] = [];
  noteDialog = false;
  currentNote: Note | null = null;
  currentNoteIndex: number | undefined = 0;

  // computed
  // lifecycle
  async created() {
    if (this.archive) {
      await this.fetchArchive();
    } else {
      await this.fetchNotes();
    }
    if (this.notes.length > 0) {
      this.currentNote = this.notes[0]; // eslint-disable-line
      this.currentNoteIndex = 0;
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

  selectedNoteChanged(selected: any) {
    if (selected !== undefined && selected !== null) {
      this.currentNote = this.notes[selected];
    } else {
      this.currentNote = null;
    }
    this.currentNoteIndex = selected;
  }
}
</script>


<style lang="scss" scoped>
.blm-main-col {
  border-left: 1px solid #dedede;
}

.v-overflow-btn .v-input__slot::before {
    border-color: grey !important;
}
</style>
