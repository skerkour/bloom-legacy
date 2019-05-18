<template>
<div>
  <v-card>
    <v-card-title primary-title @click="update" class="pointer">
      <div class="headline">{{ note.title }}</div>
    </v-card-title>
    <v-card-text @click="update" class="pointer">
      <p>{{ note.body }}</p>
    </v-card-text>
    <v-divider light></v-divider>
    <v-card-actions>
      <v-spacer />
      <v-tooltip bottom v-if="note.archived_at === null && note.removed_at == null">
        <v-btn
          flat
          icon
          slot="activator"
          color="blue-grey"
          @click="archive_note(note)"
        >
          <v-icon>mdi-package-down</v-icon>
        </v-btn>
        <span>Archive</span>
      </v-tooltip>
      <v-tooltip bottom v-else-if="note.archived_at !== null && note.removed_at == null">
        <v-btn
          flat
          icon
          slot="activator"
          color="blue-grey"
          @click="unarchive_note(note)"
        >
          <v-icon>mdi-package-up</v-icon>
        </v-btn>
        <span>Unarchive</span>
      </v-tooltip>
      <v-tooltip bottom v-if="note.removed_at === null">
        <v-btn
          flat
          icon
          slot="activator"
          color="blue-grey"
          @click="remove_note(note)"
        >
          <v-icon>mdi-delete</v-icon>
        </v-btn>
        <span>Delete</span>
      </v-tooltip>
      <v-tooltip bottom v-else>
        <v-btn
          flat
          icon
          slot="activator"
          color="blue-grey"
          @click="restore_note(note)"
        >
          <v-icon>mdi-delete-restore</v-icon>
        </v-btn>
        <span>Restore</span>
      </v-tooltip>
      <v-tooltip bottom>
        <v-menu slot="activator">
          <v-btn slot="activator" flat color="blue-grey" icon>
            <v-icon>mdi-dots-vertical</v-icon>
          </v-btn>

          <v-list>
            <v-list-tile @click="delete_note">Delete forever</v-list-tile>
          </v-list>
        </v-menu>
      <span>More actions</span>
      </v-tooltip>
    </v-card-actions>
  </v-card>
  <blm-notes-dialog-note
    :visible="dialog"
    :note="note"
    @close="dialog_closed"
    @update="note_updated"
  />
</div>
</template>


<script lang="ts">
import { Component, Vue, Prop } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';
import NoteDialog from './NoteDialog.vue';


@Component({
  components: {
    'blm-notes-dialog-note': NoteDialog,
  },
})
export default class Note extends Vue {
  // props
  @Prop({ type: Object, required: true }) note!: any | null;


  // data
  dialog = false;
  error = '';
  is_loading = false;


  // computed
  // lifecycle
  // watch
  // methods
  async archive_note() {
    this.error = '';
    this.is_loading = true;
    try {
      await api.post(`${api.NOTES}/v1/notes/${this.note.id}/archive`);
      this.$emit('archive', this.note);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  async unarchive_note() {
    this.error = '';
    this.is_loading = true;
    try {
      await api.post(`${api.NOTES}/v1/notes/${this.note.id}/unarchive`);
      this.$emit('unarchive', this.note);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  async remove_note() {
    this.error = '';
    this.is_loading = true;
    try {
      await api.post(`${api.NOTES}/v1/notes/${this.note.id}/remove`);
      this.$emit('remove', this.note);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  async restore_note() {
    this.error = '';
    this.is_loading = true;
    try {
      await api.post(`${api.NOTES}/v1/notes/${this.note.id}/restore`);
      this.$emit('restore', this.note);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  async delete_note() {
    this.error = '';
    this.is_loading = true;
    try {
      await api.delete(`${api.NOTES}/v1/notes/${this.note.id}`);
      this.$emit('delete', this.note);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  update() {
    this.dialog = true;
  }

  dialog_closed() {
    this.dialog = false;
  }

  note_updated(note: any) {
    this.$emit('update', note);
  }
}
</script>


<style scoped lang="scss">
.v-card {
  border-radius: 8px;
}
</style>
