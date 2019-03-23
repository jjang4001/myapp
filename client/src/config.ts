interface Config {
  [key: string]: EnvConfig
};

interface EnvConfig {
  doodleSocket: string
};

const config: Config = {
  development: {
    doodleSocket: 'ws://localhost:5000/doodle'
  },
  production: {
    doodleSocket: 'ws://junwonjang.dev/api/doodle'
  }
}

export default config;
