<script setup lang="ts">
const open = defineModel<boolean>();

const dark = useDark();
const name = await useStorageText("name", "");

const { t, locale, locales, setLocale } = useI18n();
</script>

<template>
  <md-dialog :open="open" @closed="open = false">
    <span slot="headline">
      {{ t("components.settings.title") }}
    </span>

    <form slot="content" method="dialog" class="flex flex-col gap-4">
      <label class="flex items-center justify-between gap-4">
        {{ t("components.settings.form.dark-theme") }}

        <md-switch
          :aria-label="t('components.settings.form.dark-theme')"
          icons
          :selected="dark"
          @change="dark = $event.target.selected"
        />
      </label>

      <md-outlined-text-field
        :label="t('components.settings.form.name')"
        :value="name"
        @input="name = $event.target.value"
      />

      <md-outlined-select
        :label="t('components.settings.form.language')"
        :value="locale"
        @input="setLocale($event.target.value)"
      >
        <md-select-option
          v-for="{ code } in locales"
          :key="code"
          :value="code"
          :selected="code === locale"
        >
          <span slot="headline">
            {{ t(`app.locales.${code}`) }}
          </span>
        </md-select-option>
      </md-outlined-select>
    </form>
  </md-dialog>
</template>
