<template>
  <div>
    <v-card>
      <v-card-title
        primary-title
        @click="update"
        class="pointer"
      >
        <div class="headline">{{ note.title }}</div>
      </v-card-title>
      <v-card-text
        @click="update"
        class="pointer"
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
          <!--  && note.removed_at == null" -->
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
          v-else-if="note.archived_at !== null"
        >
        <template v-slot:activator="{ on }">
          <!-- // && note.removed_at == null" -->
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
        <!-- <v-tooltip
          bottom
          v-if="note.removed_at === null"
        >
          <v-btn
            text
            icon
            slot="activator"
            color="blue-grey"
            @click="removeNote(note)"
          >
            <v-icon>mdi-delete</v-icon>
          </v-btn>
          <span>Delete</span>
        </v-tooltip>
        <v-tooltip
          bottom
          v-else
        >
          <v-btn
            text
            icon
            slot="activator"
            color="blue-grey"
            @click="restoreNote(note)"
          >
            <v-icon>mdi-delete-restore</v-icon>
          </v-btn>
          <span>Restore</span>
        </v-tooltip> -->
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
      @close="dialogClosed"
      @update="noteUpdated"
    />
  </div>
</template>


<script lang="ts">
import { Component, Vue, Prop } from 'vue-property-decorator';
import NoteDialog from './NoteDialog.vue';
import { Note as NoteModel, GuiNote } from '@/native/messages/notes';
import { Native, Message } from '@/native';

@Component({
  components: {
    'blm-notes-dialog-note': NoteDialog,
  },
})
export default class Note extends Vue {
  // props
  @Prop({ type: Object, required: true }) note!: NoteModel;

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
    try {
      // await api.post(`${api.NOTES}/v1/notes/${this.note.id}/archive`);
      // this.$emit('archive', this.note);
      console.log('archiveNote');
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
  }

  async unarchiveNote() {
    this.error = '';
    this.isLoading = true;
    try {
      // await api.post(`${api.NOTES}/v1/notes/${this.note.id}/unarchive`);
      // this.$emit('unarchive', this.note);
      console.log('unarchiveNote');
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
  }

  async removeNote() {
    this.error = '';
    this.isLoading = true;
    try {
      // await api.post(`${api.NOTES}/v1/notes/${this.note.id}/remove`);
      // this.$emit('remove', this.note);
      console.log('removeNote');
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
  }

  async restoreNote() {
    this.error = '';
    this.isLoading = true;
    try {
      // await api.post(`${api.NOTES}/v1/notes/${this.note.id}/restore`);
      // this.$emit('restore', this.note);
      console.log('restoreNote');
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
  }

  async deleteNote() {
    this.error = '';
    this.isLoading = true;
    const message: Message = {
      type: 'notes.gui.delete_note',
      data: {
        id: this.note.id,
      },
    };
    try {
      const res = await Native.call(message);
      this.$emit('delete', this.note);
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
    this.$emit('update', note);
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
