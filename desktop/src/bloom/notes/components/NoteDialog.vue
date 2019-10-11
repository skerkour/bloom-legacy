<template>
  <v-dialog
    v-model="show"
    @keydown.esc="show = false"
    scrollable
    width="50%"
    :fullscreen="$vuetify.breakpoint.smAndDown"
  >
    <v-card>
      <v-card-title class="headline">
        <v-text-field
          placeholder="Title"
          :disabled="isLoading"
          v-model="title"
        ></v-text-field>
      </v-card-title>
      <v-card-text>
        <v-alert icon="mdi-alert-circle" :value="error !== ''" type="error" dismissible>
          {{ error }}
        </v-alert>
        <v-textarea
          ref="notebody"
          v-model="body"
          placeholder="Take a note..."
          auto-grow
          color="white"
        ></v-textarea>
      </v-card-text>

      <!-- <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn text @click="close" :disabled="isLoading">Cancel</v-btn>
        <v-btn
          color="primary"
          :loading="isLoading"
          @click="save"
        >
          Save
        </v-btn>
      </v-card-actions> -->
    </v-card>
  </v-dialog>
</template>


<script lang="ts">
import {
  Component,
  Prop,
  Vue,
  Watch,
} from 'vue-property-decorator';
import { Note as NoteModel, GuiNote } from '@/native/messages/notes';
import { Native, Message } from '@/native';

const { log } = require('@bloom42/astro');

@Component
export default class NoteDialog extends Vue {
  // props
  @Prop({ type: Boolean, default: false }) visible!: boolean;
  @Prop({ type: Object, default: null }) note!: NoteModel | null;

  // data
  error = '';
  isLoading = false;
  title = '';
  body = '';

  // computed
  get show() {
    return this.visible;
  }

  set show(value) {
    if (!value) {
      this.save();
      this.$emit('close');
    }
  }

  // lifecycle
  created() {
    if (this.note) {
      this.title = this.note.title;
      this.body = this.note.body;
    }
  }

  // watch
  // @Watch('visible')
  // onIsVisibleChanged(isVisible: boolean) {
  //   if (isVisible) {
  //     this.$nextTick((this.$refs.notebody as any).focus);
  //   }
  // }

  // methods
  save() {
    if (this.note) {
      this.updateNote();
    } else {
      this.createNote();
    }
  }

  async createNote() {
    this.error = '';
    this.isLoading = true;
    if (this.body.length === 0 || this.title.length === 0) {
      return;
    }
    const message: Message = {
      type: 'notes.gui.create_note',
      data: {
        title: this.title,
        body: this.body,
        color: 4294967295,
      },
    };
    try {
      const res = await Native.call(message);
      this.$emit('create', (res.data as GuiNote).note);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
  }

  async updateNote() {
    this.error = '';
    this.isLoading = true;
    const note = { ...this.note } as NoteModel;
    note.title = this.title;
    note.body = this.body;
    const message: Message = {
      type: 'notes.gui.update_note',
      data: {
        note,
      },
    };
    try {
      const res = await Native.call(message);
      this.$emit('update', (res.data as GuiNote).note);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
  }

  close() {
    if (this.note === null) {
      this.title = '';
      this.body = '';
    }
    this.show = false;
  }
}
</script>


<style scoped lang="scss">
.notes-dialog-note {
  max-width: 50%;
}
</style>
