interface Config {
  [key: string]: EnvConfig
};

interface EnvConfig {
  doodleSocket: string
};

const config: Config = {
  development: {
    // doodleSocket: 'wss://192.168.99.100/api/doodle'
    doodleSocket: 'ws://localhost:5000/doodle'
  },
  production: {
    doodleSocket: 'wss://junwonjang.dev/api/doodle'
  }
}

export default config;
