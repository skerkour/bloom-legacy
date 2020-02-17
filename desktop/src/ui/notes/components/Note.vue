<template>
  <div>
    <v-card>
      <v-card-title
        primary-title
        @click="update"
        class="blm-pointer"
      >
        <div class="headline">{{ note.title }}</div>
      </v-card-title>
      <v-card-text
        @click="update"
        class="blm-pointer"
      >
        <p class="blm-note-body">{{ note.body }}</p>
      </v-card-text>
      <v-divider light></v-divider>
      <v-card-actions>
        <v-spacer />

        <v-tooltip
          bottom
          v-if="note.archived_at === null"
        >
          <template v-slot:activator="{ on }">
          <v-btn
            text
            icon
            slot="activator"
            color="blue-grey"
            @click="archiveNote(note)"
            v-on="on"
          >
            <v-icon>mdi-package-down</v-icon>
          </v-btn>
          </template>
          <span>Archive</span>
        </v-tooltip>
        <v-tooltip
          bottom
          v-else
        >
        <template v-slot:activator="{ on }">
          <v-btn
            text
            icon
            slot="activator"
            color="blue-grey"
            @click="unarchiveNote(note)"
            v-on="on"
          >
            <v-icon>mdi-package-up</v-icon>
          </v-btn>
        </template>
          <span>Unarchive</span>
        </v-tooltip>


        <v-tooltip
          bottom
          v-if="note.is_pinned"
        >
          <template v-slot:activator="{ on }">
          <v-btn
            text
            icon
            slot="activator"
            color="blue-grey"
            @click="unpinNote(note)"
            v-on="on"
          >
            <v-icon>mdi-pin</v-icon>
          </v-btn>
          </template>
          <span>Unpin</span>
        </v-tooltip>
        <v-tooltip
          bottom
          v-else
        >
        <template v-slot:activator="{ on }">
          <v-btn
            text
            icon
            slot="activator"
            color="blue-grey"
            @click="pinNote(note)"
            v-on="on"
          >
            <v-icon>mdi-pin-outline</v-icon>
          </v-btn>
        </template>
          <span>Pin</span>
        </v-tooltip>

        <v-tooltip bottom>
          <template v-slot:activator="{ on: tooltip }">
          <v-menu slot="activator">
            <template v-slot:activator="{ on: menu }">
            <v-btn
              slot="activator"
              text
              color="blue-grey"
              icon
              v-on="{ ...tooltip, ...menu }"
            >
              <v-icon>mdi-dots-vertical</v-icon>
            </v-btn>
            </template>

            <v-list>
              <v-list-item @click="deleteNote">
                <v-list-item-title>Delete forever</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-menu>
          </template>
          <span>More actions</span>
        </v-tooltip>
      </v-card-actions>
    </v-card>
    <blm-notes-dialog-note
      :visible="dialog"
      :note="note"
      @closed="dialogClosed"
      @updated="noteUpdated"
    />
  </div>
</template>


<script lang="ts">
import { Component, Vue, Prop } from 'vue-property-decorator';
import NoteDialog from './NoteDialog.vue';
import { Note, DeleteNote, Method } from '@/core/notes';
import core from '@/core';

@Component({
  components: {
    'blm-notes-dialog-note': NoteDialog,
  },
})
export default class BlmNote extends Vue {
  // props
  @Prop({ type: Object, required: true }) note!: Note;

  // data
  dialog = false;
  error = '';
  isLoading = false;

  // computed
  // lifecycle
  // watch
  // methods
  async archiveNote() {
    this.error = '';
    this.isLoading = true;
    const note = { ...this.note } as Note;
    note.archived_at = new Date().toISOString() as unknown as Date;

    try {
      const res = await core.call(Method.UpdateNote, note);
      this.$emit('archived', (res as Note));
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
  }

  async unarchiveNote() {
    this.error = '';
    this.isLoading = true;
    const note = { ...this.note } as Note;
    note.archived_at = null;

    try {
      const res = await core.call(Method.UpdateNote, note);
      this.$emit('unarchived', (res as Note));
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
  }

  async pinNote() {
    this.error = '';
    this.isLoading = true;
    const note = { ...this.note } as Note;
    note.is_pinned = true;

    try {
      const res = await core.call(Method.UpdateNote, note);
      this.$emit('updated', (res as Note));
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
  }

  async unpinNote() {
    this.error = '';
    this.isLoading = true;
    const note = { ...this.note } as Note;
    note.is_pinned = false;

    try {
      const res = await core.call(Method.UpdateNote, note);
      this.$emit('updated', (res as Note));
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
  }

  async deleteNote() {
    this.error = '';
    this.isLoading = true;
    const params: DeleteNote = {
      id: this.note.id,
    };

    try {
      await core.call(Method.DeleteNote, params);
      this.$emit('deleted', this.note);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
  }

  update() {
    this.dialog = true;
  }

  dialogClosed() {
    this.dialog = false;
  }

  noteUpdated(note: any) {
    this.$emit('updated', note);
  }
}
</script>


<style scoped lang="scss">
.v-card {
  border-radius: 8px;

  .v-card__title {
    height: 72px;
  }
}

.blm-note-body {
  height: 80px;
  text-overflow: ellipsis;
  white-space: pre;
  overflow: hidden;
}
</style>
