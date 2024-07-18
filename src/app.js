// app.js
var mainApp = angular.module("mainApp", []);

mainApp.controller("ProgressBarController", [
    "$scope",
    function ($scope) {
        var ctrl = this;
        ctrl.progress = 0;
        var unlisten;

        async function setupProgressListener() {
            unlisten = await window.__TAURI__.event.listen(
                "progress",
                (event) => {
                    console.info(parseFloat(event.payload));
                    ctrl.progress = parseFloat(event.payload);
                    $scope.$apply(); // Обновление AngularJS scope
                }
            );
        }

        ctrl.runJob = function () {
            ctrl.progress = 0;
            window.__TAURI__.invoke("long_running_job");
        };

        setupProgressListener();

        $scope.$on("$destroy", function () {
            if (unlisten) {
                unlisten();
            }
        });
    },
]);

mainApp.service("TimeService", [
    "$q",
    function ($q) {
        this.getCurrentTime = function () {
            var deferred = $q.defer();
            window.__TAURI__
                .invoke("get_current_time")
                .then(function (time) {
                    deferred.resolve(time);
                })
                .catch(function (error) {
                    deferred.reject(error);
                });
            return deferred.promise;
        };
    },
]);

mainApp.service("BreedService", [
    "$q",
    function ($q) {
        this.getBreeds = function () {
            var deferred = $q.defer();
            window.__TAURI__
                .invoke("get_breeds")
                .then(function (breeds) {
                    console.log(
                        "Response from get_breeds:",
                        JSON.parse(breeds)
                    ); // Вывод в консоль
                    deferred.resolve(breeds);
                })
                .catch(function (error) {
                    console.error("Error fetching breeds:", error);
                    deferred.reject(error);
                });
            return deferred.promise;
        };
    },
]);

mainApp.controller("TimeController", [
    "$scope",
    "TimeService",
    "$interval",
    function ($scope, TimeService, $interval) {
        function updateTime() {
            TimeService.getCurrentTime()
                .then(function (time) {
                    $scope.currentTime = time;
                })
                .catch(function (error) {
                    console.error("Error fetching time:", error);
                });
        }

        updateTime();
        $interval(updateTime, 1000);
    },
]);

mainApp.controller("BreedController", [
    "$scope",
    "BreedService",
    function ($scope, BreedService) {
        $scope.breeds = [
            {
                name: "--Select Breed--",
                name_short: "UN",
            },
            {
                name: "Boergoat",
                name_short: "BBG",
            },
            {
                name: "Damara",
                name_short: "DAM",
            },
            {
                name: "Dohne Merino",
                name_short: "DMS",
            },
            {
                name: "Dormer",
                name_short: "DOM",
            },
            {
                name: "Dorper",
                name_short: "DOP",
            },
            {
                name: "Ile de France",
                name_short: "IDF",
            },
            {
                name: "Kalahari Red",
                name_short: "KRG",
            },
            {
                name: "Meatmaster",
                name_short: "MMS",
            },
            {
                name: "Merino",
                name_short: "MER",
            },
            {
                name: "Merino Landskaap",
                name_short: "MLS",
            },
            {
                name: "SA Mutton Merino",
                name_short: "SAM",
            },
            {
                name: "Savannah Goats",
                name_short: "SSG",
            },
            {
                name: "Suffolk",
                name_short: "SUF",
            },
            {
                name: "Van Rooy",
                name_short: "VRS",
            },
            {
                name: "Wit Dorper",
                name_short: "DOPW",
            },
            {
                name: "Customize Your Own Index",
                name_short: "own",
            },
        ];

        $scope.selectedBreed = $scope.breeds[0].name_short; // Set default selection

        $scope.refreshBreeds = function () {
            BreedService.getBreeds()
                .then(function (breeds) {
                    $scope.breeds = JSON.parse(breeds);
                    // Проверка на наличие первого элемента и его свойства name_short
                    if (
                        $scope.breeds.length > 0 &&
                        $scope.breeds[0].name_short
                    ) {
                        $scope.selectedBreed = $scope.breeds[0].name_short; // Set default selection
                    } else {
                        $scope.selectedBreed = null; // Или другое значение по умолчанию
                    }

                    // $scope.selectedBreed = breeds[0].name_short; // Set default selection
                })
                .catch(function (error) {
                    console.error("Error fetching breeds:", error);
                });
        };
        console.log($scope.selectedBreed);
        // Проверка значения selectedBreed
        $scope.$watch("selectedBreed", function (newVal, oldVal) {
            console.log("selectedBreed changed from", oldVal, "to", newVal);
        });
    },
]);

mainApp.controller('MainCtrl', ['$scope', function($scope) {
    $scope.label_text = "Обработай меня";

    const { invoke } = window.__TAURI__.tauri;

    // Функция для генерации UUID v4
    const uuidv4 = () => ([1e7]+-1e3+-4e3+-8e3+-1e11).replace(/[018]/g, c =>
        (c ^ crypto.getRandomValues(new Uint8Array(1))[0] & 15 >> c / 4).toString(16)
    );

    // Функция для отправки сообщения на бекенд при нажатии кнопки
    $scope.sendToBackend = function() {
        const message = {
            function: "click_button",
            data: { text: $scope.label_text },
            uuid4: uuidv4(),
            response_queue: "backend_to_frontend"
        };
        invoke('frontend_to_backend', { payload: JSON.stringify(message) });
    };

    // Слушатель для получения сообщений от бекенда
    window.addEventListener('backend_to_frontend', function(event) {
        const message = JSON.parse(event.detail);
        if (message.function === "front_click_button") {
            $scope.$apply(function() {
                $scope.label_text = message.data.processed_data;
            });
        }
    });
}]);