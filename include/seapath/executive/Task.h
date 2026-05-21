#pragma once
#include <string>

namespace seapath::executive {

class ITask {
public:
    virtual ~ITask() = default;
    
    virtual void on_start() = 0;
    virtual void update() = 0;
    virtual void on_stop() = 0;
    
    virtual std::string get_name() const = 0;
};

} // namespace seapath::executive