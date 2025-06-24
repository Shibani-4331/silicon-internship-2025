# 1-Week Rust Projects

## Minor Project: Intelligent File Deduplicator

### Core Features
- **Multi-Algorithm Hashing**: SHA-256, Blake3, and xxHash for different use cases
- **Parallel Processing**: Rayon-based parallel file processing with progress tracking
- **Advanced Filtering**: Size ranges, file types, date ranges, regex patterns
- **Safe Operations**: Quarantine system before actual deletion
- **Detailed Reports**: JSON/HTML reports with file relationships and savings

### Advanced Features
- **Content Similarity**: Compare text files using edit distance algorithms
- **Image Similarity**: Perceptual hashing for images using image crate
- **Incremental Scanning**: Only scan changed files using modification times and checksums
- **Recovery System**: Maintain deletion history with rollback capabilities
- **Space Analysis**: Detailed breakdown of potential space savings

### Advanced Challenges
- **Sophisticated Grouping**: Group similar files by content, not just exact matches
- **Performance Optimization**: Memory-mapped files, efficient hash computation
- **Advanced Verification**: Multiple verification passes before deletion
- **Metadata Analysis**: Consider file attributes, EXIF data for better deduplication
- **Custom Algorithms**: Implement domain-specific similarity detection

---

## Additional Project Ideas [Not a Part of the Minor Project] 

### 1. Advanced Process Resource Monitor

#### Core Features
- **Process Tree Tracking**: Monitor parent-child process relationships using sysinfo crate
- **Multi-Threaded Data Collection**: Separate threads for data collection, analysis, and display (Optional - If multithreading is taught)
- **SQLite Local Storage**: Store historical data with rusqlite for trend analysis
- **Configurable Thresholds**: TOML config files with hot-reloading using notify crate
- **Rich Terminal UI**: Real-time TUI using ratatui with multiple views

#### Advanced Features
- **Statistical Analysis**: Calculate moving averages, detect spikes, and trend analysis
- **Alert System**: Local desktop notifications using notify-rust
- **Process Filtering**: Complex filters (by name, user, resource usage patterns)
- **Data Export**: Generate CSV/JSON reports with serde
- **Resource Prediction**: Simple linear regression to predict resource exhaustion

#### Advanced Challenges
- **Memory Leak Detection**: Track memory growth patterns over time
- **Anomaly Detection**: Statistical outlier detection for unusual resource usage
- **Process Correlation**: Identify processes that consume resources together
- **Performance Optimization**: Efficient data structures, memory pooling
- **Configuration Management**: Layered config (default → user → runtime)

### 2. Comprehensive System Health Monitor

#### Core Features
- **Multi-Metric Collection**: CPU, memory, disk I/O, network, temperature using multiple crates
- **Terminal Dashboard**: Split-pane TUI with real-time graphs and metrics
- **Historical Data**: Efficient circular buffers for different time ranges (1min, 1hr, 24hr)
- **Alert Engine**: Rule-based alerting with severity levels and cooldown periods
- **Report Generation**: HTML reports with embedded charts using plotters

#### Advanced Features
- **Predictive Analysis**: Forecast system resource exhaustion using trend analysis
- **Custom Metrics**: Plugin-like system for user-defined metrics via configuration
- **Baseline Learning**: Automatically learn "normal" system behavior patterns
- **Correlation Analysis**: Identify relationships between different system metrics
- **Performance Profiling**: Self-monitoring and optimization of the monitor itself

#### Advanced Challenges
- **Adaptive Monitoring**: Adjust collection frequency based on system load
- **Smart Alerting**: Machine learning-like pattern recognition for reducing false positives
- **Resource Optimization**: Minimize monitoring overhead through efficient sampling
- **Cross-Platform Adaptation**: Handle OS-specific metrics and behaviors
- **Data Compression**: Efficient storage of historical data with compression