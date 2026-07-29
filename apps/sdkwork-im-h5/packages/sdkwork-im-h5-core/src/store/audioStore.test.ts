import assert from "node:assert/strict";
import test, { afterEach } from "node:test";

import { useAudioStore } from "./audioStore";

const originalAudio = globalThis.Audio;

class FakeAudioElement extends EventTarget {
  static instances: FakeAudioElement[] = [];

  currentTime = 0;
  duration = 120;
  loadCount = 0;
  src = "";

  constructor() {
    super();
    FakeAudioElement.instances.push(this);
  }

  load(): void {
    this.loadCount += 1;
  }

  pause(): void {
    this.dispatchEvent(new Event("pause"));
  }

  play(): Promise<void> {
    this.dispatchEvent(new Event("play"));
    return Promise.resolve();
  }

  removeAttribute(name: string): void {
    if (name === "src") {
      this.src = "";
    }
  }
}

afterEach(() => {
  Object.defineProperty(globalThis, "Audio", {
    configurable: true,
    value: originalAudio,
    writable: true,
  });
  useAudioStore.getState().stop();
  useAudioStore.setState({ audioElement: null });
  FakeAudioElement.instances = [];
});

test("creates audio lazily, reuses it and releases media on stop", () => {
  installFakeAudio();
  const track = {
    id: "track-1",
    title: "Track",
    artist: "Artist",
    coverUrl: "https://media.example.test/cover.png",
    audioUrl: "https://media.example.test/track.mp3",
  };

  assert.equal(useAudioStore.getState().audioElement, null);

  useAudioStore.getState().playMusic(track);
  useAudioStore.getState().playMusic(track);

  const audio = FakeAudioElement.instances[0];
  assert.ok(audio);
  assert.equal(FakeAudioElement.instances.length, 1);
  assert.equal(audio.src, track.audioUrl);
  assert.equal(useAudioStore.getState().currentTrack?.id, track.id);
  assert.equal(useAudioStore.getState().isPlaying, true);

  useAudioStore.getState().stop();

  assert.equal(audio.src, "");
  assert.equal(audio.loadCount, 1);
  assert.equal(useAudioStore.getState().currentTrack, null);
  assert.equal(useAudioStore.getState().duration, 0);
  assert.equal(useAudioStore.getState().progress, 0);
});

test("seek clamps finite input to the loaded media range", () => {
  installFakeAudio();
  useAudioStore.getState().initAudio();
  const audio = FakeAudioElement.instances[0];
  assert.ok(audio);

  useAudioStore.getState().seek(150);
  assert.equal(audio.currentTime, 120);
  assert.equal(useAudioStore.getState().progress, 120);

  useAudioStore.getState().seek(-10);
  assert.equal(audio.currentTime, 0);
  assert.equal(useAudioStore.getState().progress, 0);

  useAudioStore.getState().seek(Number.NaN);
  assert.equal(audio.currentTime, 0);
});

function installFakeAudio(): void {
  Object.defineProperty(globalThis, "Audio", {
    configurable: true,
    value: FakeAudioElement,
    writable: true,
  });
}
