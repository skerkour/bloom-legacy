<template>
  <div>
    <v-container fluid grid-list-lg>
      <v-layout row wrap justify-left class="mt-1">
      <v-flex
        v-for="note in notes"
        xs12
        sm4
        md3
        :key="note.id"
      >
      <blm-notes-note
        :note="note"
        @unarchive="note_unarchived"
        @update="note_updated"
        @remove="note_removed"
        @delete="note_deleted"
      />
      </v-flex>
    </v-layout>
    </v-container>

  <blm-notes-dialog-note
    :visible="note_dialog"
    @close="note_dialog_closed"
    @create="note_created"
  />
  </div>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';
import Note from './Note.vue';
import NoteDialog from './NoteDialog.vue';


@Component({
  components: {
    'blm-notes-dialog-note': NoteDialog,
    'blm-notes-note': Note,
  },
})
export default class Notes extends Vue {
  // props
  // data
  error = '';
  is_loading = false;
  notes: any[] = [];
  note_dialog = false;

  // computed
  // lifecycle
  created() {
    this.fetch_data();
  }


  // watch
  // methods
  async fetch_data() {
    this.error = '';
    this.is_loading = true;
    try {
      this.notes = await api.get(`${api.NOTES}/v1/archive`);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  open_note_dialog() {
    this.note_dialog = true;
  }

  note_dialog_closed() {
    this.note_dialog = false;
  }

  note_created(note: any) {
    this.notes = [note, ...this.notes];
  }

  note_updated(updated_note: any) {
    this.notes = this.notes.map((note: any) => {
      if (note.id === updated_note.id) {
        return updated_note;
      }
      return note;
    });
  }

  note_unarchived(archived_note: any) {
    this.notes = this.notes.filter((note) => note.id !== archived_note.id);
  }

  note_removed(removed_note: any) {
    this.notes = this.notes.filter((note) => note.id !== removed_note.id);
  }

  note_deleted(deleted_note: any) {
    this.notes = this.notes.filter((note) => note.id !== deleted_note.id);
  }
}
</script>


<style scoped lang="scss">
</style>
