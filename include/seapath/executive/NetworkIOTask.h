#pragma once
#include "Task.h"
#include "TelemetryFrame.h"
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <unistd.h>
#include <cstring>
#include <iostream>

namespace seapath::executive {

class NetworkIOTask : public ITask {
private:
    std::string name;
    int socket_fd;
    struct sockaddr_in local_addr{};
    struct sockaddr_in target_addr{};
    
    uint8_t receive_buffer[128];
    TelemetryFrame current_frame{};

public:
    NetworkIOTask(std::string n) : name(std::move(n)), socket_fd(-1) {}

    ~NetworkIOTask() override {
        if (socket_fd >= 0) {
            close(socket_fd);
        }
    }

    void on_start() override {
        std::cout << "  [IO] Opening UDP socket line...\n";
        
        socket_fd = socket(AF_INET, SOCK_DGRAM, 0);
        if (socket_fd < 0) {
            std::cerr << "  [IO CRITICAL] Failed to create socket.\n";
            return;
        }

        // Listen on Port 8080 (Where Rust sends its data)
        local_addr.sin_family = AF_INET;
        local_addr.sin_addr.s_addr = htonl(INADDR_ANY);
        local_addr.sin_port = htons(8080);

        // Target Port 8081 (Where Rust listens for replies)
        target_addr.sin_family = AF_INET;
        target_addr.sin_addr.s_addr = inet_addr("127.0.0.1");
        target_addr.sin_port = htons(8081);

        if (bind(socket_fd, (struct sockaddr*)&local_addr, sizeof(local_addr)) < 0) {
            std::cerr << "  [IO CRITICAL] Socket bind failed on port 8080.\n";
            return;
        }

        // Critical for Real-Time Loops: Set a ultra-short timeout 
        // so if Rust crashes, our C++ scheduler loop doesn't freeze waiting for data.
        struct timeval tv;
        tv.tv_sec = 0;
        tv.tv_usec = 1000; // 1ms max timeout
        setsockopt(socket_fd, SOL_SOCKET, SO_RCVTIMEO, &tv, sizeof(tv));
        
        std::cout << "  [IO] Socket bound to port 8080. Non-blocking guard active.\n";
    }

    void update() override {
        if (socket_fd < 0) return;

        socklen_t addr_len = sizeof(target_addr);
        ssize_t bytes_read = recvfrom(socket_fd, receive_buffer, sizeof(receive_buffer), 0,
                                      (struct sockaddr*)&target_addr, &addr_len);

        // If we got a valid frame size, unpack it and read the data
        if (bytes_read >= 56) {
            std::memcpy(&current_frame, receive_buffer, sizeof(TelemetryFrame));
            
            std::cout << "[HIL INGEST] Seq: " << current_frame.sequence_number 
                      << " | Heading: " << current_frame.heading_rad << " rad"
                      << " | Depth: " << current_frame.current_depth_m << " m\n";

            // Mock responses to echo back to the Rust Sim terminal
            current_frame.rudder_deg = -0.15f;     // Mocking a slight port turn command
            current_frame.stern_plane_deg = 0.05f;  // Mocking a minor depth correction

            sendto(socket_fd, &current_frame, sizeof(TelemetryFrame), 0,
                   (struct sockaddr*)&target_addr, sizeof(target_addr));
        }
    }

    void on_stop() override {
        if (socket_fd >= 0) {
            close(socket_fd);
            socket_fd = -1;
        }
        std::cout << "  [IO] Socket line closed cleanly.\n";
    }

    std::string get_name() const override { return name; }
};

} // namespace seapath::executive