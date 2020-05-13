<template>
  <div>
    <v-toolbar elevation="0" v-if="note">
      <v-text-field
        v-model="note.data.title"
        placeholder="Title"
        hide-details
        solo
        flat
        class="headline"
      />

      <v-menu>
        <template v-slot:activator="{ on }">
          <v-btn icon v-on="on">
            <v-icon>mdi-dots-vertical</v-icon>
          </v-btn>
        </template>
        <v-list class="text-left">
          <!-- <v-list-item v-if="!note.isPinned">
            <v-list-item-icon>
              <v-icon>mdi-pin</v-icon>
            </v-list-item-icon>
            <v-list-item-title>Pin</v-list-item-title>
          </v-list-item>
          <v-list-item v-else>
            <v-list-item-icon>
              <v-icon>mdi-pin-outline</v-icon>
            </v-list-item-icon>
            <v-list-item-title>Unpin</v-list-item-title>
          </v-list-item> -->
          <v-list-item v-if="note.data.archivedAt === null" @click="archiveNote">
            <v-list-item-icon>
              <v-icon>mdi-package-down</v-icon>
            </v-list-item-icon>
            <v-list-item-title>Archive</v-list-item-title>
          </v-list-item>
          <v-list-item v-else @click="unarchiveNote">
            <v-list-item-icon>
              <v-icon>mdi-package-up</v-icon>
            </v-list-item-icon>
            <v-list-item-title>Unarchive</v-list-item-title>
          </v-list-item>
          <v-list-item @click="deleteNote">
            <v-list-item-icon>
              <v-icon>mdi-delete</v-icon>
            </v-list-item-icon>
            <v-list-item-title>Delete forever</v-list-item-title>
          </v-list-item>
        </v-list>
      </v-menu>
    </v-toolbar>

    <div class="overflow-y-auto" v-if="note">
      <v-textarea
        v-model="note.data.body"
        placeholder="Take a note..."
        autofocus
        hide-details
        solo
        flat
        height="calc(100vh - 65px)"
        no-resize
      ></v-textarea>
    </div>
  </div>
</template>


<script lang="ts">
import {
  Component,
  Vue,
  Prop,
  // Watch,
} from 'vue-property-decorator';
import { Note, DeleteNote, Method } from '@/core/notes';
import core, { BlmObject } from '@/core';

@Component
export default class BlmNote extends Vue {
  // props
  @Prop({ type: Object, required: true }) note!: BlmObject;

  // data
  error = '';
  isLoading = false;

  // computed
  // lifecycle
  // watch
  // methods
  async archiveNote() {
    this.error = '';
    this.isLoading = true;
    const note = { ...this.note } as BlmObject;
    note.data.archivedAt = new Date().toISOString() as unknown as Date;

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
    const note = { ...this.note } as BlmObject;
    note.data.archivedAt = null;

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
    const note = { ...this.note } as BlmObject;
    note.data.isPinned = true;

    try {
      const res = await core.call(Method.UpdateNote, note);
      this.$emit('updated', (res as BlmObject));
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
  }

  async unpinNote() {
    this.error = '';
    this.isLoading = true;
    const note = { ...this.note } as BlmObject;
    note.data.isPinned = false;

    try {
      const res = await core.call(Method.UpdateNote, note);
      this.$emit('updated', (res as BlmObject));
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

  noteUpdated(note: any) {
    this.$emit('updated', note);
  }
}
</script>


<style scoped lang="scss">
.blm-note-body {
  height: 80px;
  text-overflow: ellipsis;
  white-space: pre;
  overflow: hidden;
}

.v-toolbar {
  border-bottom: 1px solid rgba($color: #000000, $alpha: 0.1) !important;
  left: 0px !important;
}
</style>
