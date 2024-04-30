{ config
, options
, lib
, pkgs
, ...
}:

let
  cfg = config.services.aaura-w3-strawb;
in {
  options.services.aaura-w3-strawb = {
    enable = lib.mkEnableOption "The service to run Anna Auror's dynamic webserver, serving Anna's personal website.";
    bind = lib.mkOption {
      description = "The address, including port, to bind the webserver to.";
      type = lib.types.nullOr lib.types.str;
      example = "0.0.0.0:47238";
      default = null;
    };
    webData = lib.mkOption {
      description = "A directory containing files served by the webserver directly or processed at a different route.";
      type = lib.types.nullOr lib.types.path;
      default = null;
    };
    bcdgJson = lib.mkOption {
      description = "A path to a file containing the JSON for the Be crime do gay webring. It is used for determining the nearest nodes in the ring.";
      type = lib.types.nullOr lib.types.path;
      default = null;
    };
  };

  config.fonts.packages = lib.mkIf cfg.enable (with pkgs; [
    comic-neue
  ]);

  config.users = lib.mkIf cfg.enable {
    groups.aaura-w3-strawb = {};
    users.aaura-w3-strawb = {
      group = "aaura-w3-strawb";
      isSystemUser = true;
    };
  };

  config.systemd.services.aaura-w3-strawb = lib.mkIf cfg.enable rec {
    enable = true;
    description = "dynamic webserver for Anna Aurora's website";
    wantedBy = [ "multi-user.target" ];
    environment = {}
    // (if cfg.bind == null then {} else {
      AAURA_W3_STRAWB_BIND_ADDRESS = cfg.bind;
    })
    // (if cfg.webData == null then {} else {
      AAURA_W3_STRAWB_WEB_DATA_DIR = toString cfg.webData;
    })
    // (if cfg.bcdgJson == null then {} else {
      AAURA_W3_STRAWB_BCDG_JSON_PATH = toString cfg.bcdgJson;
    });

    serviceConfig = {
      User = "aaura-w3-strawb";
      Group = "aaura-w3-strawb";
      StateDirectory = [ "aaura-w3-strawb" ];

      ExecStart = "${pkgs.aaura-w3-strawb}/bin/aaura-w3-strawb";

      Restart = "on-failure";

      # Hardening
      CapabilityBoundingSet = [ "" ];
      LockPersonality = true;
      PrivateDevices = true;
      PrivateUsers = true;
      ProcSubset = "pid";
      ProtectSystem = "strict";
      ProtectClock = true;
      ProtectControlGroups = true;
      ProtectHome = true;
      ProtectHostname = true;
      ProtectKernelLogs = true;
      ProtectKernelModules = true;
      ProtectKernelTunables = true;
      ProtectProc = "invisible";
      RestrictAddressFamilies = [ "AF_INET" "AF_INET6" ];
      RestrictNamespaces = true;
      RestrictRealtime = true;
      SystemCallArchitectures = "native";
      SystemCallFilter = [
        "@system-service"
        "~@privileged @aio @chown @keyring @memlock @resources @setuid @timer memfd_create"
      ];
      # No UMask because this service doesn't create files
      RestrictSUIDSGID = true;
      RemoveIPC = true;
      NoNewPrivileges = true;
      MemoryDenyWriteExecute = true;
      NoExecPaths = [ "/" ];
      ExecPaths = [ "/nix/store" ];
      # Needs /run/nscd
      InaccessiblePaths = [ "/sys" "/dev/shm" "/run/dbus" "/run/user" ];
      # Exits with code 244 if filesystems are restricted
      RestrictFileSystems = [];
    };
  };
}
