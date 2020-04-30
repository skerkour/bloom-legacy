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
        v-model="selectedNoteIndex"
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
  CreateNote,
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
  saveInterval: any | null = null;

  // computed
  // lifecycle
  async created() {
    // this.saveInterval = setInterval(() => {
    //   this.save();
    // }, 2000);
    if (this.archive) {
      await this.findArchived();
    } else {
      await this.findNotes();
    }
    this.setSelectedNoteIndex(0);
  }

  destroyed() {
    if (this.saveInterval) {
      clearInterval(this.saveInterval);
    }
  }

  // methods
  async save() {
    if (this.selectedNote) {
      if (this.selectedNote.id === '') {
        await this.createNote();
      } else {
        await this.updateNote();
      }
    }
  }

  async findNotes() {
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

  async findArchived() {
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

  // noteCreated(note: Note) {
  //   this.notes = [note, ...this.notes];
  // }

  noteUpdated(updatedNote: Note) {
    // const pos = this.notes.map((note: Note) => note.id).indexOf(updatedNote.id);
    // this.notes.splice(pos, 1);
    // this.notes = [updatedNote, ...this.notes];
    this.notes[this.selectedNoteIndex!] = updatedNote;
    this.selectedNote = updatedNote;


    // this.notes = this.notes.map((note: any) => {
    //   if (note.id === updatedNote.id) {
    //     return updatedNote;
    //   }
    //   return note;
    // });
  }

  noteArchived(archivedNote: Note) {
    this.notes = this.notes.filter((note: Note) => note.id !== archivedNote.id);
    this.selectedNote = null;
    this.setSelectedNoteIndex(0);
  }

  noteUnarchived(unarchivedNote: Note) {
    this.notes = this.notes.filter((note: Note) => note.id !== unarchivedNote.id);
    this.selectedNote = null;
    this.setSelectedNoteIndex(0);
  }

  noteDeleted(deletedNote: Note) {
    this.notes = this.notes.filter((note: Note) => note.id !== deletedNote.id);
    this.selectedNote = null;
    this.setSelectedNoteIndex(0);
  }

  async setSelectedNoteIndex(selected: number | undefined) {
    // save before changing / closing note
    await this.save();

    if (selected === undefined || selected >= this.notes.length) {
      this.selectedNoteIndex = undefined;
      this.selectedNote = null;
    } else {
      const note = this.notes[selected];
      this.notes.splice(selected, 1);
      this.selectedNote = note;
      this.notes = [note, ...this.notes];
      this.selectedNoteIndex = 0;
    }
  }

  async newNote() {
    await this.setSelectedNoteIndex(undefined);

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
    await this.setSelectedNoteIndex(0);
  }

  async createNote() {
    this.error = '';
    this.isLoading = true;
    const params: CreateNote = {
      title: this.selectedNote!.title,
      body: this.selectedNote!.body,
      color: '#ffffff',
    };
    try {
      const res = await core.call(Method.CreateNote, params);
      this.notes[0] = res;
      this.selectedNote = res;
      // this.selectedNote = res;
      // this.$emit('created', (res as Note));
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
  }

  async updateNote() {
    this.error = '';
    this.isLoading = true;
    const note = { ...this.selectedNote } as Note;
    try {
      const res = await core.call(Method.UpdateNote, note);
      this.notes[0] = res;
      this.selectedNote = res;
      // this.selectedNote = res;
      // this.$emit('updated', (res as Note));
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
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
