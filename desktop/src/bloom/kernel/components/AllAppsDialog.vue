<template>
  <v-dialog
    v-model="show"
    fullscreen
    hide-overlay
    dark
  >
    <v-card>
      <v-toolbar elevation="0">
        <v-btn
          icon
          dark
          @click="show = false"
        >
          <v-icon>mdi-close</v-icon>
        </v-btn>
      </v-toolbar>

      <v-card-text>
        <v-row>
          <v-col offset="3" cols="6">
            <v-container>

              <v-row justify="start">
                <v-col cols="2" v-for="(app, index) in apps1" :key="index" class="blm-app-icon">
                  <v-avatar class="blm-pointer" @click="goto(app.path)" size="52">
                    <img :src="app.icon" :alt="app.name" />
                  </v-avatar>
                  <p class="subtitle-1 font-weight-medium blm-app-icon-name">{{ app.name }}</p>
                </v-col>
              </v-row>

              <v-row justify="start">
                <v-col cols="2" v-for="(app, index) in apps2" :key="index" class="blm-app-icon">
                  <v-avatar class="blm-pointer" @click="goto(app.path)" size="52">
                    <img :src="app.icon" :alt="app.name" />
                  </v-avatar>
                  <p class="subtitle-1 font-weight-medium blm-app-icon-name">{{ app.name }}</p>
                </v-col>
              </v-row>

            </v-container>
          </v-col>
        </v-row>
      </v-card-text>

    </v-card>
  </v-dialog>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';

@Component
export default class AllAppsDialog extends Vue {
  // props
  @Prop({ type: Boolean, default: false }) visible!: boolean;


  // data
  apps1 = [
    {
      name: 'Chat',
      path: '/chat',
      icon: '/kernel/imgs/icons/chat_512.jpg',
    },
    {
      name: 'Music',
      path: '/music',
      icon: '/kernel/imgs/icons/music_512.jpg',
    },
    {
      name: 'Notes',
      path: '/notes',
      icon: '/kernel/imgs/icons/notes_512.jpg',
    },
    {
      name: 'Calendar',
      path: '/calendar',
      icon: '/kernel/imgs/icons/calendar_512.jpg',
    },
    {
      name: 'Bitflow',
      path: '/bitflow',
      icon: '/kernel/imgs/icons/bitflow_512.jpg',
    },
  ];
  apps2 = [
    {
      name: 'Drive',
      path: '/drive',
      icon: '/kernel/imgs/icons/drive_512.jpg',
    },
    {
      name: 'Arcade',
      path: '/arcade',
      icon: '/kernel/imgs/icons/arcade_512.jpg',
    },
    {
      name: 'Contacts',
      path: '/contacts',
      icon: '/kernel/imgs/icons/contacts_512.jpg',
    },
    {
      name: 'Gallery',
      path: '/gallery',
      icon: '/kernel/imgs/icons/gallery_512.jpg',
    },
  ];

  // computed
  get show() {
    return this.visible;
  }

  set show(value) {
    if (!value) {
      this.$emit('closed');
    }
  }


  // lifecycle
  // watch
  // methods
  goto(path: string) {
    this.show = false;
    this.$router.push({ path }).catch((_) => {});
  }
}
</script>


<style lang="scss" scoped>
.blm-app-icon {
  overflow-x: visible;
  white-space: nowrap;
}

.blm-app-icon-name {
  margin-top: 10px;
}
</style>
