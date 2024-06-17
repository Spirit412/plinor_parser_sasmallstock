// app.js
var mainApp = angular.module("mainApp", []);

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
