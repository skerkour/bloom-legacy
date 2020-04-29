<template>
  <v-layout fill-height>
    <v-col cols="4" class="pa-0">
      <v-toolbar elevation="0" v-if="!archive">
         <v-spacer />
        <v-tooltip bottom>
          <template v-slot:activator="{ on }">
            <v-btn icon v-on="on" @click="newNote">
              <v-icon>mdi-plus</v-icon>
            </v-btn>
          </template>
          <span>New Note</span>
        </v-tooltip>
      </v-toolbar>

      <v-list-item-group
        :value="selectedNoteIndex"
        @change="setSelectedNoteIndex"
        color="indigo">
        <v-list three-line class="overflow-y-auto pa-0">
          <template v-for="(note, index) in notes" class="blm-pointer">
            <v-list-item :key="`note-${index}`">

              <v-list-item-content class="text-left">
                <v-list-item-title>{{ note.title }}</v-list-item-title>
                <v-list-item-subtitle>{{ note.body }}</v-list-item-subtitle>
              </v-list-item-content>

            </v-list-item>
            <v-divider v-if="index !== notes.length - 1" :key="index"/>
          </template>
        </v-list>
      </v-list-item-group>
    </v-col>

    <v-col cols="8" class="pa-0 blm-main-col">
      <blm-notes-note v-if="selectedNote"
        :note="selectedNote"
        @archived="noteArchived"
        @unarchived="noteUnarchived"
        @updated="noteUpdated"
        @deleted="noteDeleted"
      />
    </v-col>
  </v-layout>
</template>


<script lang="ts">
import {
  Component,
  Prop,
  Vue,
  // Watch,
} from 'vue-property-decorator';
import BlmNote from './Note.vue';
import core from '@/core';
import {
  Note,
  Notes,
  Method,
} from '@/core/notes';
import { log } from '@/libs/rz';

@Component({
  components: {
    'blm-notes-note': BlmNote,
  },
})
export default class BlmNotes extends Vue {
  // props
  @Prop({ type: Boolean, default: false }) archive!: boolean;

  // data
  error = '';
  isLoading = false;
  notes: Note[] = [];
  selectedNote: Note | null = null;
  selectedNoteIndex: number | undefined = 0;

  // computed
  // lifecycle
  async created() {
    if (this.archive) {
      await this.fetchArchive();
    } else {
      await this.fetchNotes();
    }
    if (this.notes.length > 0) {
      this.selectedNote = this.notes[0]; // eslint-disable-line
      this.selectedNoteIndex = 0;
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

  // methods
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

  setSelectedNoteIndex(selected: any) {
    if (selected !== undefined && selected !== null) {
      this.selectedNote = this.notes[selected];
    } else {
      this.selectedNote = null;
    }
    this.selectedNoteIndex = selected;
  }

  newNote() {
    const newNote: Note = {
      id: '',
      createdAt: new Date(),
      updatedAt: new Date(),
      title: '',
      body: '',
      color: '#ffffff',
      archivedAt: null,
      isPinned: false,
    };
    this.notes = [newNote, ...this.notes];
    this.setSelectedNoteIndex(0);
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
