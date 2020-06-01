<template>
  <v-dialog
    v-model="show"
    @keydown.esc="close()"
    @click:outside="close()"
    scrollable
    width="50%"
    :fullscreen="$vuetify.breakpoint.smAndDown"
  >
    <v-card>
      <v-card-title>
        <h2 class="headline">Invite people</h2>
      </v-card-title>

      <v-card-text>
        <v-col cols="12" v-if="error">
          <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
            {{ error }}
          </v-alert>
        </v-col>

        <v-col cols="12">
          <!-- <v-combobox
            v-model="usersToInvite"
            label="Usernames"
            multiple
            chips
            :loading="loading"
        ></v-combobox> -->
        <v-text-field
           v-model="userToInvite"
           :loading="loading"
           placeholder="Username"
           prefix="@"
           @keyup.enter.native="invite"
        />
        </v-col>

      </v-card-text>
      <v-card-actions>
        <v-spacer />
        <v-btn text @click="cancel()" :loading="loading">
          Cancel
        </v-btn>
        <v-btn color="primary" @click="invite()" :loading="loading">
          Invite
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>


<script lang="ts">
import { Component, Vue, Prop } from 'vue-property-decorator';
import { Group } from '@/api/models';
import core from '@/core';
import { Method } from '@/core/groups';
import { GroupsInviteUserParams } from '@/core/messages';


@Component
export default class Groups extends Vue {
  // props
  @Prop({ type: Boolean, default: false }) visible!: boolean;
  @Prop({ type: String, default: false }) groupID!: string;

  // data
  error = '';
  loading = false;
  userToInvite = '';

  // computed
  get show() {
    return this.visible;
  }

  set show(value) {
    if (!value) {
      this.$emit('closed');
    }
  }

  // computed
  // lifecycle
  // watch
  // methods
  async invite() {
    this.loading = true;
    this.error = '';
    const params: GroupsInviteUserParams = {
      groupID: this.groupID,
      username: this.userToInvite,
    };

    try {
      const res: Group = await core.call(Method.InviteUser, params);
      this.cancel();
      this.$emit('invited', res.invitations);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.loading = false;
    }
  }

  cancel() {
    this.userToInvite = '';
    this.error = '';
    this.loading = false;
    this.close();
  }

  async close() {
    this.show = false;
  }
}
</script>


<style lang="scss" scoped>
</style>
